use super::Query;
use crate::error::Result;
/// 数字专辑-语种风格馆
/// 对应 Node.js module/album_list_style.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 数字专辑-语种风格馆
    /// 对应 /album/list/style
    pub async fn album_list_style(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "10").parse::<i64>().unwrap_or(10),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true,
            "area": query.get_or("area", "Z_H")
        });
        self.request(
            "/api/vipmall/appalbum/album/style",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
