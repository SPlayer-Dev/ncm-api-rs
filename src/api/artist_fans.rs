use super::Query;
use crate::error::Result;
/// 歌手粉丝
/// 对应 Node.js module/artist_fans.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手粉丝
    /// 对应 /artist/fans
    pub async fn artist_fans(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0"),
            "limit": query.get_or("limit", "20"),
            "offset": query.get_or("offset", "0")
        });
        self.request(
            "/api/artist/fans/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
