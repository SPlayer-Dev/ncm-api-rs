/// HTTP 服务模块 - 对应 Node.js 版本的 server.js
///
/// 使用 Axum 框架，将 ApiClient 的所有方法自动映射为 REST API 路由
/// 前端可以像调用 Node.js 版一样通过 HTTP 请求调用
pub mod middleware;
pub mod upload;

use crate::api::Query;
use crate::request::{ApiClient, ApiResponse};
use axum::extract::State;
use axum::http::header;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// ============================================================
//  服务器配置
// ============================================================

/// 服务器配置
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// 监听地址，默认 "0.0.0.0"
    pub host: String,
    /// 监听端口，默认 3000
    pub port: u16,
    /// CORS 允许的 Origin，None 表示允许所有
    pub cors_origin: Option<String>,
    /// 限流：时间窗口内最大请求数，0 表示不限流
    pub rate_limit: u64,
    /// 限流：时间窗口秒数
    pub rate_limit_window: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            cors_origin: None,
            rate_limit: 0,
            rate_limit_window: 60,
        }
    }
}

impl ServerConfig {
    /// 从环境变量读取配置
    /// - `NCM_HOST`: 监听地址
    /// - `NCM_PORT`: 监听端口
    /// - `CORS_ALLOW_ORIGIN`: CORS 允许的 Origin
    /// - `RATE_LIMIT`: 每个时间窗口的最大请求数（默认不限流）
    /// - `RATE_LIMIT_WINDOW`: 限流时间窗口秒数（默认 60）
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("NCM_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("NCM_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            cors_origin: std::env::var("CORS_ALLOW_ORIGIN").ok(),
            rate_limit: std::env::var("RATE_LIMIT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(0),
            rate_limit_window: std::env::var("RATE_LIMIT_WINDOW")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(60),
        }
    }
}

// ============================================================
//  共享状态
// ============================================================

/// Axum 共享状态
#[derive(Clone)]
pub struct AppState {
    pub client: Arc<ApiClient>,
}

// ============================================================
//  查询参数提取
// ============================================================

/// 从 HTTP 请求中提取并合并查询参数
///
/// 合并优先级（后者覆盖前者）：Cookie < URL Query < POST Body
async fn extract_merged_query(
    headers: &HeaderMap,
    uri_query: Option<&str>,
    body: axum::body::Bytes,
    content_type: Option<&str>,
) -> Query {
    let mut query = Query::new();

    // 1. 提取 URL query 参数
    if let Some(qs) = uri_query {
        if let Ok(params) = serde_urlencoded::from_str::<HashMap<String, String>>(qs) {
            for (k, v) in params {
                query.params.insert(k, v);
            }
        }
    }

    // 2. 提取 POST body 参数
    if !body.is_empty() {
        let ct = content_type.unwrap_or("");
        if ct.contains("application/json") {
            if let Ok(map) = serde_json::from_slice::<HashMap<String, Value>>(&body) {
                for (k, v) in map {
                    let s = match &v {
                        Value::String(s) => s.clone(),
                        _ => v.to_string(),
                    };
                    query.params.insert(k, s);
                }
            }
        } else {
            // form-urlencoded 或其他格式
            if let Ok(params) = serde_urlencoded::from_bytes::<HashMap<String, String>>(&body) {
                for (k, v) in params {
                    query.params.insert(k, v);
                }
            }
        }
    }

    // 3. 处理 cookie
    // 优先使用参数中传入的 cookie
    if let Some(cookie_param) = query.params.remove("cookie") {
        query.cookie = Some(cookie_param);
    } else if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(c) = cookie_header.to_str() {
            query.cookie = Some(c.to_string());
        }
    }

    // 4. 处理 realIP
    if let Some(real_ip) = query.params.remove("realIP") {
        query.real_ip = Some(real_ip);
    }

    // 5. 处理 proxy
    if let Some(proxy) = query.params.remove("proxy") {
        query.proxy = Some(proxy);
    }

    // 6. 其余请求选项（对应 Node.js util/option.js）
    // randomCNIP 保留在 params 中，供 ENABLE_RANDOM_CN_IP 判断显式的 false
    query.random_cn_ip = query.get("randomCNIP") == Some("true");
    if let Some(e_r) = query.params.remove("e_r") {
        query.e_r = Some(e_r == "true");
    }
    if let Some(ua) = query.params.remove("ua").filter(|s| !s.is_empty()) {
        query.ua = Some(ua);
    }
    if let Some(domain) = query.params.remove("domain").filter(|s| !s.is_empty()) {
        query.domain = Some(domain);
    }
    if let Some(timeout) = query.params.remove("timeout") {
        query.timeout = timeout.parse().ok();
    }
    if let Some(headers) = query.params.remove("headers") {
        if let Ok(Value::Object(map)) = serde_json::from_str::<Value>(&headers) {
            query.headers = map
                .into_iter()
                .map(|(k, v)| match v {
                    Value::String(s) => (k, s),
                    other => (k, other.to_string()),
                })
                .collect();
        }
    }

    query
}

/// 客户端 IP 透传（对应 Node.js server.js，trust proxy 开启）：
/// 未显式传 realIP、未开启 randomCNIP 时，转发客户端 IP（优先 X-Forwarded-For）；
/// 本机请求改用进程级随机中国 IP
fn apply_client_ip(query: &mut Query, headers: &HeaderMap, peer: Option<std::net::IpAddr>) {
    if query.real_ip.is_some() || query.random_cn_ip {
        return;
    }
    let forwarded = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .and_then(|v| v.trim().parse::<std::net::IpAddr>().ok());
    let ip = match forwarded.or(peer) {
        Some(std::net::IpAddr::V6(v6)) => v6
            .to_ipv4_mapped()
            .map(std::net::IpAddr::V4)
            .unwrap_or(std::net::IpAddr::V6(v6)),
        Some(ip) => ip,
        None => return,
    };
    if ip.is_loopback() {
        query.random_cn_ip = true;
    } else {
        query.real_ip = Some(ip.to_string());
    }
}

// ============================================================
//  响应构建
// ============================================================

/// 构建成功响应
fn build_success_response(api_resp: ApiResponse) -> Response {
    let status = axum::http::StatusCode::from_u16(api_resp.status as u16)
        .unwrap_or(axum::http::StatusCode::OK);
    let mut response = (status, Json(api_resp.body)).into_response();

    // 设置 API 返回的 Cookie
    for cookie_str in &api_resp.cookie {
        if let Ok(val) = header::HeaderValue::from_str(cookie_str) {
            response.headers_mut().append(header::SET_COOKIE, val);
        }
    }

    response
}

/// 构建错误响应
fn build_error_response(err: crate::error::NcmError) -> Response {
    use crate::error::NcmError;

    // 上游错误：原样返回 body 和 cookie（与 Node.js 版一致，301 时 msg 改为「需要登录」）
    if let NcmError::Response { response, .. } = err {
        let mut resp = *response;
        if resp.status == 301 {
            if let Some(obj) = resp.body.as_object_mut() {
                obj.insert("msg".into(), json!("需要登录"));
            }
        }
        return build_success_response(resp);
    }

    let (status, body) = match &err {
        NcmError::AuthRequired(msg) => (
            axum::http::StatusCode::UNAUTHORIZED,
            json!({ "code": 301, "msg": msg }),
        ),
        NcmError::InvalidParam(msg) => (
            axum::http::StatusCode::BAD_REQUEST,
            json!({ "code": 400, "msg": msg }),
        ),
        NcmError::RateLimited(msg) => (
            axum::http::StatusCode::TOO_MANY_REQUESTS,
            json!({ "code": 503, "msg": msg }),
        ),
        NcmError::Timeout(msg) => (
            axum::http::StatusCode::GATEWAY_TIMEOUT,
            json!({ "code": 504, "msg": msg }),
        ),
        NcmError::Api { code, msg } => {
            let http_status = axum::http::StatusCode::from_u16(*code as u16)
                .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
            (http_status, json!({ "code": code, "msg": msg }))
        }
        _ => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "code": 500, "msg": err.to_string() }),
        ),
    };

    (status, Json(body)).into_response()
}

// ============================================================
//  通用请求处理
// ============================================================

/// 通用 API 请求处理函数
/// 客户端连接地址（服务以 `into_make_service_with_connect_info` 启动时可用）
type Peer = Option<axum::Extension<axum::extract::ConnectInfo<std::net::SocketAddr>>>;

async fn handle_api_request<F>(
    state: &AppState,
    headers: HeaderMap,
    peer: Peer,
    uri: &axum::http::Uri,
    body: axum::body::Bytes,
    api_fn: F,
) -> Response
where
    F: for<'a> FnOnce(
        &'a ApiClient,
        &'a Query,
    ) -> Pin<
        Box<dyn Future<Output = crate::error::Result<ApiResponse>> + Send + 'a>,
    >,
{
    let path = uri.path().to_string();
    let start = std::time::Instant::now();

    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok());

    let mut query = extract_merged_query(&headers, uri.query(), body, content_type).await;
    apply_client_ip(&mut query, &headers, peer.map(|p| p.0 .0.ip()));

    match api_fn(&state.client, &query).await {
        Ok(resp) => {
            tracing::info!("{} -> {} ({:.1?})", path, resp.status, start.elapsed());
            build_success_response(resp)
        }
        Err(e) => {
            tracing::warn!("{} -> ERROR: {} ({:.1?})", path, e, start.elapsed());
            build_error_response(e)
        }
    }
}

// ============================================================
//  路由注册宏
// ============================================================

/// 批量注册 API 路由的宏
///
/// 将每个 ApiClient 方法映射为 GET + POST 路由
macro_rules! api_routes {
    ($router:expr, $( $method:ident => $route:expr ),* $(,)?) => {{
        let router = $router;
        $(
            let router = router.route(
                $route,
                get(|State(state): State<AppState>, headers: HeaderMap, peer: Peer, uri: axum::http::Uri| async move {
                    handle_api_request(&state, headers, peer, &uri, axum::body::Bytes::new(), |client, q| Box::pin(client.$method(q))).await
                })
                .post(|State(state): State<AppState>, headers: HeaderMap, peer: Peer, uri: axum::http::Uri, body: axum::body::Bytes| async move {
                    handle_api_request(&state, headers, peer, &uri, body, |client, q| Box::pin(client.$method(q))).await
                }),
            );
        )*
        router
    }};
}

/// 注册单条自定义路由（GET + POST），`$f` 签名同 `handle_api_request` 的 api_fn
macro_rules! custom_route {
    ($router:expr, $route:expr, $f:expr) => {
        $router.route(
            $route,
            get(
                |State(state): State<AppState>,
                 headers: HeaderMap,
                 peer: Peer,
                 uri: axum::http::Uri| async move {
                    handle_api_request(&state, headers, peer, &uri, axum::body::Bytes::new(), $f)
                        .await
                },
            )
            .post(
                |State(state): State<AppState>,
                 headers: HeaderMap,
                 peer: Peer,
                 uri: axum::http::Uri,
                 body: axum::body::Bytes| async move {
                    handle_api_request(&state, headers, peer, &uri, body, $f).await
                },
            ),
        )
    };
}

// ============================================================
//  路由注册
// ============================================================

/// 注册所有 API 路由
///
/// 标准路由由 build.rs 自动从 src/api/mod.rs 扫描生成
/// 上传路由（avatar_upload, voice_upload）需要 multipart 处理，单独注册
fn register_routes(router: Router<AppState>) -> Router<AppState> {
    // 自动生成的标准路由（build.rs → api_routes_generated.rs）
    let router = { include!(concat!(env!("OUT_DIR"), "/api_routes_generated.rs")) };

    // 文件上传路由（特殊签名，需要 multipart 处理）
    let router = router
        .route("/avatar/upload", post(upload::handle_avatar_upload))
        .route("/voice/upload", post(upload::handle_voice_upload));

    // 非标准签名的方法（无 query / 同步方法）
    let router = custom_route!(router, "/inner/version", |client, _q| Box::pin(
        client.inner_version()
    ));
    let router = custom_route!(router, "/eapi/decrypt", |client, q| Box::pin(
        std::future::ready(client.eapi_decrypt(q))
    ));

    router
}

// ============================================================
//  构建与启动
// ============================================================

/// 构建 Axum 应用（不启动监听）
///
/// 可用于测试或自定义部署
pub fn build_app(client: ApiClient) -> Router {
    let state = AppState {
        client: Arc::new(client),
    };

    let router = Router::new();
    let router = register_routes(router);

    // 根路由
    let router = router.route(
        "/",
        get(|| async {
            Json(json!({
                "code": 200,
                "msg": "NCM API Rust Server is running",
            }))
        }),
    );

    router.layer(middleware::cors_layer(None)).with_state(state)
}

/// 构建带自定义配置的 Axum 应用
pub fn build_app_with_config(client: ApiClient, config: &ServerConfig) -> Router {
    let state = AppState {
        client: Arc::new(client),
    };

    let router = Router::new();
    let router = register_routes(router);

    let router = router.route(
        "/",
        get(|| async {
            Json(json!({
                "code": 200,
                "msg": "NCM API Rust Server is running",
            }))
        }),
    );

    let router = router
        .layer(middleware::cors_layer(config.cors_origin.as_deref()))
        .with_state(state);

    // 限流中间件（rate_limit > 0 时启用）
    if config.rate_limit > 0 {
        let limiter = middleware::RateLimiter::new(config.rate_limit, config.rate_limit_window);
        router.layer(axum::middleware::from_fn_with_state(
            limiter,
            middleware::rate_limit_middleware,
        ))
    } else {
        router
    }
}

/// 启动 HTTP 服务器
pub async fn start_server(config: ServerConfig) {
    let mut client = ApiClient::new(None);
    // 与 Node.js 版启动时 generateConfig 一致：注册游客身份，未登录请求携带 MUSIC_A
    match client.register_anonimous(&Query::new()).await {
        Ok(resp) => {
            let token = resp.cookie.iter().find_map(|c| {
                c.split(';')
                    .find_map(|kv| kv.trim().strip_prefix("MUSIC_A="))
                    .map(str::to_string)
            });
            match token {
                Some(token) => {
                    tracing::info!("游客 token 注册成功");
                    client.set_anonymous_token(token);
                }
                None => tracing::warn!("游客 token 注册未返回 MUSIC_A: {}", resp.body),
            }
        }
        Err(e) => tracing::warn!("游客 token 注册失败: {}", e),
    }
    let app = build_app_with_config(client, &config);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    tracing::info!("NCM API Server listening on http://{}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .expect("Server error");
}
