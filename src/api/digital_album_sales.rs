use super::Query;
use crate::error::Result;
/// 数字专辑销量
/// 对应 Node.js module/digitalAlbum_sales.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 数字专辑销量
    /// 对应 /digitalAlbum/sales
    pub async fn digital_album_sales(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "albumIds": query.get_or("ids", "")
        });
        self.request(
            "/api/vipmall/albumproduct/album/query/sales",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
