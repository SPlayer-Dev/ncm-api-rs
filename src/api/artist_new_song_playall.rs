use super::Query;
use crate::error::Result;
/// 获取所有关注歌手最近的 50 首新歌
/// 对应 Node.js module/artist_new_song_playall.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取所有关注歌手最近的 50 首新歌
    /// 对应 /artist/new/song/playall
    pub async fn artist_new_song_playall(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/sub/artist/new/works/song/playall",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
