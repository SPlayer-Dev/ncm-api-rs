use super::Query;
use crate::error::Result;
/// 专辑简要百科信息
/// 对应 Node.js module/ugc_album_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 专辑简要百科信息
    /// 对应 /ugc/album/get
    pub async fn ugc_album_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "albumId": query.get_or("id", "")
        });
        self.request(
            "/api/rep/ugc/album/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
