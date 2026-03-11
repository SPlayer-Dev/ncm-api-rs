use super::Query;
use crate::error::Result;
/// 歌词摘录 - 添加/修改摘录歌词
/// 对应 Node.js module/song_lyrics_mark_add.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌词摘录 - 添加/修改摘录歌词
    /// 对应 /song/lyrics/mark/add
    pub async fn song_lyrics_mark_add(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "0"),
            "markId": query.get_or("markId", ""),
            "data": query.get_or("data", "[]"),
        });
        self.request(
            "/api/song/play/lyrics/mark/add",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
