use super::Query;
use crate::error::Result;
/// 私信歌单
/// 对应 Node.js module/send_playlist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 私信歌单
    /// 对应 /send/playlist
    pub async fn send_playlist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("playlist", "0"),
            "type": "playlist",
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
