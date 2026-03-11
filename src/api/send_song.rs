use super::Query;
use crate::error::Result;
/// 私信歌曲
/// 对应 Node.js module/send_song.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 私信歌曲
    /// 对应 /send/song
    pub async fn send_song(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0"),
            "msg": query.get_or("msg", ""),
            "type": "song",
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
