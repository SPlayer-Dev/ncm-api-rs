use super::Query;
use crate::error::Result;
/// 不喜欢推荐歌曲
/// 对应 Node.js module/recommend_songs_dislike.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 不喜欢推荐歌曲
    /// 对应 /recommend/songs/dislike
    pub async fn recommend_songs_dislike(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "resId": query.get_or("id", "0"),
            "resType": 4,
            "sceneType": 1
        });
        self.request(
            "/api/v2/discovery/recommend/dislike",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
