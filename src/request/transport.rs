//! HTTP 连接池策略：复用连接，但赶在服务端回收前丢弃空闲连接，复用失败时换新连接重试一次。
//!
//! interface*.music.163.com 会在连接空闲约 5 秒后关闭它，复用中的连接也偶尔被重置（issue #2）。
//! 完全不复用（每次请求都重新 TCP + TLS 握手）能避开这个问题，但每个请求要多付 0.4–0.5 秒，
//! 一个命令要打多次上游时成倍变慢。这里改为：
//! - 空闲连接 [`POOL_IDLE_TIMEOUT`] 后主动丢弃，先于服务端的 5 秒回收；
//! - 仍撞上失效连接（请求发出后没拿到任何响应）时，[`send_with_retry`] 换一条连接再发一次。
//!
//! TLS 自带 rustls 配置（ring + 内置 webpki 根证书 + ALPN http/1.1），不依赖进程级默认加密后端；
//! 只走 HTTP/1.1、不让 reqwest 自动协商压缩：同进程里别的库给 reqwest 开了 http2 / gzip 等
//! 特性时，这里的请求行为也保持不变。

use std::sync::Arc;
use std::time::Duration;

/// 空闲连接的保留时长：必须短于服务端约 5 秒的空闲回收。
pub(crate) const POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(4);

/// HTTP 客户端构造器（连接复用 + 空闲回收）。所有 reqwest 客户端都必须由此创建。
pub(crate) fn client_builder() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .tls_backend_preconfigured(tls_config())
        .http1_only()
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .no_zstd()
        .pool_idle_timeout(POOL_IDLE_TIMEOUT)
}

fn tls_config() -> rustls::ClientConfig {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let roots = rustls::RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
    };
    let mut config = rustls::ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .expect("ring supports the default TLS versions")
        .with_root_certificates(roots)
        .with_no_client_auth();
    config.alpn_protocols = vec![b"http/1.1".to_vec()];
    config
}

/// 失效连接导致的发送失败：请求发出前后连接被关 / 重置，没有拿到任何响应。
/// 超时、读响应体或解码失败不算 —— 那时服务端已在处理，重发可能重复执行。
fn is_stale_connection(err: &reqwest::Error) -> bool {
    (err.is_request() || err.is_connect())
        && !err.is_timeout()
        && !err.is_body()
        && !err.is_decode()
}

/// 发送请求；失效连接导致失败时换新连接重试一次（失败的连接已被连接池丢弃）。
pub(crate) async fn send_with_retry(
    req: reqwest::RequestBuilder,
) -> Result<reqwest::Response, reqwest::Error> {
    let retry = req.try_clone();
    match req.send().await {
        Err(err) if is_stale_connection(&err) => match retry {
            Some(retry) => retry.send().await,
            None => Err(err),
        },
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    /// 读完一个请求（头 + Content-Length 正文）；连接已关返回 false。
    fn read_request(reader: &mut BufReader<TcpStream>) -> bool {
        let mut len = 0usize;
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).unwrap_or(0) == 0 {
                return false;
            }
            if let Some(v) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                len = v.trim().parse().unwrap_or(0);
            }
            if line == "\r\n" {
                break;
            }
        }
        let mut body = vec![0; len];
        reader.read_exact(&mut body).is_ok()
    }

    /// 模拟服务端回收复用连接：每条连接只答第一个请求，第二个请求到达时直接关闭、不回复。
    fn flaky_server() -> (String, Arc<AtomicUsize>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/api", listener.local_addr().unwrap());
        let connections = Arc::new(AtomicUsize::new(0));
        let count = Arc::clone(&connections);
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                count.fetch_add(1, Ordering::SeqCst);
                std::thread::spawn(move || {
                    let mut writer = stream.try_clone().unwrap();
                    let mut reader = BufReader::new(stream);
                    if read_request(&mut reader) {
                        let _ = writer.write_all(
                            b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: keep-alive\r\n\r\nok",
                        );
                    }
                    let _ = read_request(&mut reader); // 复用的第二个请求:读完即断开
                });
            }
        });
        (url, connections)
    }

    #[tokio::test]
    async fn reuses_connection_and_retries_once_when_server_drops_it() {
        let (url, connections) = flaky_server();
        let client = client_builder().build().unwrap();
        for _ in 0..3 {
            let req = client.post(&url).body("a=1");
            let resp = send_with_retry(req)
                .await
                .expect("retry on a fresh connection");
            assert_eq!(resp.text().await.unwrap(), "ok");
        }
        // 第 2、3 次都先撞上被关的复用连接,各重试一次新连接:共 3 条连接
        assert_eq!(connections.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn connection_refused_is_not_masked() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/api", listener.local_addr().unwrap());
        drop(listener);
        let client = client_builder().build().unwrap();
        let err = send_with_retry(client.post(&url).body("a=1"))
            .await
            .unwrap_err();
        assert!(err.is_connect());
    }
}
