use super::Query;
use crate::error::Result;
/// 最近播放-歌曲
/// 对应 Node.js module/record_recent_song.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 最近播放-歌曲
    /// 对应 /record/recent/song
    pub async fn record_recent_song(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "100").parse::<i64>().unwrap_or(100)
        });
        self.request(
            "/api/play-record/song/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
