use super::Query;
use crate::error::Result;
/// 已收藏专辑列表
/// 对应 Node.js module/album_sublist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 已收藏专辑列表
    /// 对应 /album/sublist
    pub async fn album_sublist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "25").parse::<i64>().unwrap_or(25),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request(
            "/api/album/sublist",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
