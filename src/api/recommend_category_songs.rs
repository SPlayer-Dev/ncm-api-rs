use super::Query;
use crate::error::Result;
/// 每日推荐风格歌曲
/// 对应 Node.js module/recommend_category_songs.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 每日推荐风格歌曲
    /// 对应 /recommend/category/songs
    pub async fn recommend_category_songs(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/homepage/category/daily/song/list",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
