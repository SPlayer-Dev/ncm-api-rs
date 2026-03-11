use super::Query;
use crate::error::Result;
/// 首页-发现 dragon ball
/// 对应 Node.js module/homepage_dragon_ball.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 首页-发现 dragon ball（需要登录或游客登录）
    /// 对应 /homepage/dragon/ball
    pub async fn homepage_dragon_ball(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/homepage/dragon/ball/static",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
