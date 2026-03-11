use super::Query;
use crate::error::Result;
/// 推荐新歌
/// 对应 Node.js module/personalized_newsong.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 推荐新歌
    /// 对应 /personalized/newsong
    pub async fn personalized_newsong(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type": "recommend",
            "limit": query.get_or("limit", "10").parse::<i64>().unwrap_or(10),
            "areaId": query.get_or("areaId", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/personalized/newsong",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
