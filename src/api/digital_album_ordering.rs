use super::Query;
use crate::error::Result;
/// 购买数字专辑
/// 对应 Node.js module/digitalAlbum_ordering.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 购买数字专辑
    /// 对应 /digitalAlbum/ordering
    pub async fn digital_album_ordering(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "business": "Album",
            "paymentMethod": query.get_or("payment", ""),
            "digitalResources": serde_json::to_string(&json!([{
                "business": "Album",
                "resourceID": query.get_or("id", ""),
                "quantity": query.get_or("quantity", "1")
            }])).unwrap_or_default(),
            "from": "web"
        });
        self.request(
            "/api/ordering/web/digital",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
