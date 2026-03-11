use super::Query;
use crate::error::Result;
/// 歌词摘录 - 歌词摘录信息
/// 对应 Node.js module/song_lyrics_mark.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌词摘录 - 歌词摘录信息
    /// 对应 /song/lyrics/mark
    pub async fn song_lyrics_mark(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "0"),
        });
        self.request(
            "/api/song/play/lyrics/mark/song",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
