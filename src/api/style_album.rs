use super::Query;
use crate::error::Result;
/// 曲风-专辑
/// 对应 Node.js module/style_album.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 曲风-专辑
    /// 对应 /style/album
    pub async fn style_album(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "cursor": query.get_or("cursor", "0").parse::<i64>().unwrap_or(0),
            "size": query.get_or("size", "20").parse::<i64>().unwrap_or(20),
            "tagId": query.get_or("tagId", ""),
            "sort": query.get_or("sort", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/style-tag/home/album",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
