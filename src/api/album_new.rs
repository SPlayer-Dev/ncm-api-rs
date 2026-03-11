use super::Query;
use crate::error::Result;
/// 全部新碟
/// 对应 Node.js module/album_new.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 全部新碟
    /// 对应 /album/new
    pub async fn album_new(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true,
            "area": query.get_or("area", "ALL")
        });
        self.request("/api/album/new", data, query.to_option(CryptoType::Weapi))
            .await
    }
}
