use super::Query;
use crate::error::Result;
/// 数字专辑详情
/// 对应 Node.js module/album_detail.js / digitalAlbum_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 数字专辑详情
    /// 对应 /album/detail
    pub async fn album_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0")
        });
        self.request(
            "/api/vipmall/albumproduct/detail",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
