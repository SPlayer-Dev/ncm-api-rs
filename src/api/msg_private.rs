use super::Query;
use crate::error::Result;
/// 私信列表
/// 对应 Node.js module/msg_private.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 私信列表
    /// 对应 /msg/private
    pub async fn msg_private(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "total": "true"
        });
        self.request(
            "/api/msg/private/users",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
