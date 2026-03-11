use super::Query;
use crate::error::Result;
/// 历史每日推荐歌曲详情
/// 对应 Node.js module/history_recommend_songs_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 历史每日推荐歌曲详情
    /// 对应 /history/recommend/songs/detail
    pub async fn history_recommend_songs_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "date": query.get_or("date", "")
        });
        self.request(
            "/api/discovery/recommend/songs/history/detail",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
