use super::Query;
use crate::error::Result;
/// 关注歌手新歌
/// 对应 Node.js module/artist_new_song.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 关注歌手新歌
    /// 对应 /artist/new/song
    pub async fn artist_new_song(&self, query: &Query) -> Result<ApiResponse> {
        let before = query.get_or("before", &chrono::Utc::now().timestamp_millis().to_string());
        let data = json!({
            "limit": query.get_or("limit", "20"),
            "startTimestamp": before
        });
        self.request(
            "/api/sub/artist/new/works/song/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
