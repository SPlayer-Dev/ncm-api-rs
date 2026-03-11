use super::Query;
use crate::error::Result;
/// 歌曲动态封面
/// 对应 Node.js module/song_dynamic_cover.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲动态封面
    /// 对应 /song/dynamic/cover
    pub async fn song_dynamic_cover(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "0"),
        });
        self.request(
            "/api/songplay/dynamic-cover",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
