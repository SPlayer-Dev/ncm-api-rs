use super::Query;
use crate::error::Result;
/// 历史推荐歌曲
/// 对应 Node.js module/history_recommend_songs.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 历史推荐歌曲
    /// 对应 /history/recommend/songs
    pub async fn history_recommend_songs(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/discovery/recommend/songs/history/recent",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
