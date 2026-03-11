use super::Query;
use crate::error::Result;
/// 热门话题
/// 对应 Node.js module/hot_topic.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 热门话题
    /// 对应 /hot/topic
    pub async fn hot_topic(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "20").parse::<i64>().unwrap_or(20),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0)
        });
        self.request("/api/act/hot", data, query.to_option(CryptoType::Weapi))
            .await
    }
}
