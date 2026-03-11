use super::Query;
use crate::error::Result;
/// @我
/// 对应 Node.js module/msg_forwards.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// @我
    /// 对应 /msg/forwards
    pub async fn msg_forwards(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "total": "true"
        });
        self.request(
            "/api/forwards/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
