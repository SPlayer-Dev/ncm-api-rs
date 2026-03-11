use super::Query;
use crate::error::Result;
/// 我的数字专辑
/// 对应 Node.js module/digitalAlbum_purchased.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 我的数字专辑
    /// 对应 /digitalAlbum/purchased
    pub async fn digital_album_purchased(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request(
            "/api/digitalAlbum/purchased",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
