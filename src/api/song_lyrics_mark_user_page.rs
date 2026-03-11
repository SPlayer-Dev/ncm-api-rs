use super::Query;
use crate::error::Result;
/// 歌词摘录 - 我的歌词本
/// 对应 Node.js module/song_lyrics_mark_user_page.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌词摘录 - 我的歌词本
    /// 对应 /song/lyrics/mark/user/page
    pub async fn song_lyrics_mark_user_page(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "10").parse::<i64>().unwrap_or(10),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
        });
        self.request(
            "/api/song/play/lyrics/mark/user/page",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
