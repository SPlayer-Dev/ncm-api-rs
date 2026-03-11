use super::Query;
use crate::error::Result;
/// 歌曲音质详情
/// 对应 Node.js module/song_music_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲音质详情
    /// 对应 /song/music/detail
    pub async fn song_music_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "0"),
        });
        self.request(
            "/api/song/music/detail/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
