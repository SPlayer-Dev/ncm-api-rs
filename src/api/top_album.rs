use super::Query;
use crate::error::Result;
/// 新碟上架
/// 对应 Node.js module/top_album.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 新碟上架
    /// 对应 /top/album
    pub async fn top_album(&self, query: &Query) -> Result<ApiResponse> {
        let now = chrono::Utc::now();
        let data = json!({
            "area": query.get_or("area", "ALL"),
            "limit": query.get_or("limit", "50").parse::<i64>().unwrap_or(50),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "type": query.get_or("type", "new"),
            "year": query.get_or("year", &now.format("%Y").to_string()),
            "month": query.get_or("month", &now.format("%-m").to_string()),
            "total": false,
            "rcmd": true
        });
        self.request(
            "/api/discovery/new/albums/area",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
