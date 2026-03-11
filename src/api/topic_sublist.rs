use super::Query;
use crate::error::Result;
/// 收藏的专栏
/// 对应 Node.js module/topic_sublist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 收藏的专栏
    /// 对应 /topic/sublist
    pub async fn topic_sublist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "50").parse::<i64>().unwrap_or(50),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request(
            "/api/topic/sublist",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
