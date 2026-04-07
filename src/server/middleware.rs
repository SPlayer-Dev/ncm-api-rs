/// 中间件配置 - CORS 和限流
use axum::http::{HeaderValue, Method};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

/// 创建 CORS 中间件层
///
/// - `origin`: 允许的 Origin，None 表示允许所有
///   支持逗号分隔多个源，例如 `"https://a.com,https://b.com"`
pub fn cors_layer(origin: Option<&str>) -> CorsLayer {
    let layer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
        .allow_credentials(false);

    match origin {
        Some(o) => {
            let origins: Vec<String> = o
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if origins.is_empty() || origins.iter().any(|s| s == "*") {
                layer.allow_origin(Any)
            } else if origins.len() == 1 {
                if let Ok(val) = origins[0].parse::<HeaderValue>() {
                    layer
                        .allow_credentials(true)
                        .allow_origin(AllowOrigin::exact(val))
                } else {
                    layer.allow_origin(Any)
                }
            } else {
                layer
                    .allow_credentials(true)
                    .allow_origin(AllowOrigin::predicate(move |value, _| {
                        value
                            .to_str()
                            .map(|v| origins.iter().any(|o| o == v))
                            .unwrap_or(false)
                    }))
            }
        }
        None => layer.allow_origin(Any),
    }
}

// ============================================================
//  简易限流中间件
// ============================================================

use axum::extract::ConnectInfo;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

/// 限流器状态
#[derive(Clone)]
pub struct RateLimiter {
    /// IP -> (请求计数, 窗口起始时间)
    state: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
    /// 时间窗口内最大请求数
    max_requests: u64,
    /// 时间窗口（秒）
    window_secs: u64,
}

impl RateLimiter {
    /// 创建限流器
    ///
    /// - `max_requests`: 时间窗口内允许的最大请求数
    /// - `window_secs`: 时间窗口秒数
    pub fn new(max_requests: u64, window_secs: u64) -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_secs,
        }
    }

    /// 检查是否允许请求，返回 true 表示放行
    async fn check(&self, ip: &str) -> bool {
        let mut state = self.state.lock().await;
        let now = Instant::now();
        let entry = state.entry(ip.to_string()).or_insert((0, now));

        // 窗口过期，重置计数
        if now.duration_since(entry.1).as_secs() >= self.window_secs {
            entry.0 = 0;
            entry.1 = now;
        }

        entry.0 += 1;
        entry.0 <= self.max_requests
    }
}

/// 限流中间件处理函数
///
/// 用法：
/// ```ignore
/// let limiter = RateLimiter::new(120, 60); // 每分钟 120 次
/// app.layer(axum::middleware::from_fn_with_state(limiter, rate_limit_middleware))
/// ```
pub async fn rate_limit_middleware(
    axum::extract::State(limiter): axum::extract::State<RateLimiter>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    // 从连接信息或 X-Forwarded-For 获取 IP
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("unknown").trim().to_string())
        .or_else(|| {
            req.extensions()
                .get::<ConnectInfo<SocketAddr>>()
                .map(|ci| ci.0.ip().to_string())
        })
        .unwrap_or_else(|| "unknown".to_string());

    if !limiter.check(&ip).await {
        tracing::warn!("Rate limited: {}", ip);
        return (
            StatusCode::TOO_MANY_REQUESTS,
            axum::Json(json!({
                "code": 429,
                "msg": "Too many requests, please slow down",
            })),
        )
            .into_response();
    }

    next.run(req).await
}
