use super::Query;
use crate::error::Result;
/// 歌手热门歌曲
/// 对应 Node.js module/artist_top_song.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手热门歌曲
    /// 对应 /artist/top/song
    pub async fn artist_top_song(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0")
        });
        self.request(
            "/api/artist/top/song",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
