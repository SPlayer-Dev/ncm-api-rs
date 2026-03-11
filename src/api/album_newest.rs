use super::Query;
use crate::error::Result;
/// 最新专辑
/// 对应 Node.js module/album_newest.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 最新专辑
    /// 对应 /album/newest
    pub async fn album_newest(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/discovery/newAlbum",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
