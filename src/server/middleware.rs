/// CORS 中间件配置
use axum::http::{HeaderValue, Method};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

/// 创建 CORS 中间件层
///
/// - `origin`: 允许的 Origin，None 表示允许所有
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
            if let Ok(val) = o.parse::<HeaderValue>() {
                layer
                    .allow_credentials(true)
                    .allow_origin(AllowOrigin::exact(val))
            } else {
                layer.allow_origin(Any)
            }
        }
        None => layer.allow_origin(Any),
    }
}
