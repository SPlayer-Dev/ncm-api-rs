use super::Query;
use crate::error::Result;
/// 私信专辑
/// 对应 Node.js module/send_album.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 私信专辑
    /// 对应 /send/album
    pub async fn send_album(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0"),
            "msg": query.get_or("msg", ""),
            "type": "album",
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
