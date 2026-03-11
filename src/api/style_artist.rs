use super::Query;
use crate::error::Result;
/// 曲风-歌手
/// 对应 Node.js module/style_artist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 曲风-歌手
    /// 对应 /style/artist
    pub async fn style_artist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "cursor": query.get_or("cursor", "0").parse::<i64>().unwrap_or(0),
            "size": query.get_or("size", "20").parse::<i64>().unwrap_or(20),
            "tagId": query.get_or("tagId", ""),
            "sort": 0
        });
        self.request(
            "/api/style-tag/home/artist",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
