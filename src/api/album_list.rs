use super::Query;
use crate::error::Result;
/// 数字专辑-新碟上架
/// 对应 Node.js module/album_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 数字专辑-新碟上架
    /// 对应 /album/list
    pub async fn album_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true,
            "area": query.get_or("area", "ALL"),
            "type": query.get_or("type", "")
        });
        self.request(
            "/api/vipmall/albumproduct/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
