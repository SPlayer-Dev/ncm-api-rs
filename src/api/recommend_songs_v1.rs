use super::Query;
use crate::error::Result;
/// 每日推荐歌曲 v1
/// 对应 Node.js module/recommend_songs_v1.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 每日推荐歌曲 v1
    /// 对应 /recommend/songs/v1
    pub async fn recommend_songs_v1(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "ispush": query.get_or("ispush", "false")
        });
        self.request(
            "/api/v3/discovery/recommend/songs",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
