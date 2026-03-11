use super::Query;
use crate::error::Result;
/// 曲风-歌曲
/// 对应 Node.js module/style_song.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 曲风-歌曲
    /// 对应 /style/song
    pub async fn style_song(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "cursor": query.get_or("cursor", "0").parse::<i64>().unwrap_or(0),
            "size": query.get_or("size", "20").parse::<i64>().unwrap_or(20),
            "tagId": query.get_or("tagId", ""),
            "sort": query.get_or("sort", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/style-tag/home/song",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
