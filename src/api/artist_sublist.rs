use super::Query;
use crate::error::Result;
/// 收藏的歌手列表
/// 对应 Node.js module/artist_sublist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 收藏的歌手列表
    /// 对应 /artist/sublist
    pub async fn artist_sublist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "25").parse::<i64>().unwrap_or(25),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request(
            "/api/artist/sublist",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
