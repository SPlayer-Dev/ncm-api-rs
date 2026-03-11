use super::Query;
use crate::error::Result;
/// 私信历史
/// 对应 Node.js module/msg_private_history.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 私信历史
    /// 对应 /msg/private/history
    pub async fn msg_private_history(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "userId": query.get_or("uid", "0"),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "time": query.get_or("before", "0").parse::<i64>().unwrap_or(0),
            "total": "true"
        });
        self.request(
            "/api/msg/private/history",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
