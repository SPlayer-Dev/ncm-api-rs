use super::Query;
use crate::error::Result;
/// 电台订阅者列表
/// 对应 Node.js module/dj_subscriber.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台订阅者列表
    /// 对应 /dj/subscriber
    pub async fn dj_subscriber(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "time": query.get_or("time", "-1"),
            "id": query.get_or("id", "0"),
            "limit": query.get_or("limit", "20"),
            "total": "true"
        });
        self.request(
            "/api/djradio/subscriber",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
