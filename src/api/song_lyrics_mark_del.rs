use super::Query;
use crate::error::Result;
/// 歌词摘录 - 删除摘录歌词
/// 对应 Node.js module/song_lyrics_mark_del.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌词摘录 - 删除摘录歌词
    /// 对应 /song/lyrics/mark/del
    pub async fn song_lyrics_mark_del(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "markIds": query.get_or("id", "0"),
        });
        self.request(
            "/api/song/play/lyrics/mark/del",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
