use super::Query;
use crate::error::Result;
/// 歌曲创作者信息
/// 对应 Node.js module/song_creators.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲创作者信息
    /// 对应 /song/creators
    pub async fn song_creators(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "")
        });
        self.request(
            "/api/song/creators",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
