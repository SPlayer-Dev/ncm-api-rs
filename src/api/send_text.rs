use super::Query;
use crate::error::Result;
/// 私信
/// 对应 Node.js module/send_text.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 私信
    /// 对应 /send/text
    pub async fn send_text(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type": "text",
            "msg": query.get_or("msg", ""),
            "userIds": format!("[{}]", query.get_or("user_ids", ""))
        });
        self.request(
            "/api/msg/private/send",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
