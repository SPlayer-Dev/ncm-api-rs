use super::Query;
use crate::error::Result;
/// 曲风-歌单
/// 对应 Node.js module/style_playlist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 曲风-歌单
    /// 对应 /style/playlist
    pub async fn style_playlist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "cursor": query.get_or("cursor", "0").parse::<i64>().unwrap_or(0),
            "size": query.get_or("size", "20").parse::<i64>().unwrap_or(20),
            "tagId": query.get_or("tagId", ""),
            "sort": 0
        });
        self.request(
            "/api/style-tag/home/playlist",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
