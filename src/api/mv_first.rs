use super::Query;
use crate::error::Result;
/// 最新 MV
/// 对应 Node.js module/mv_first.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 最新 MV
    /// 对应 /mv/first
    pub async fn mv_first(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "area": query.get_or("area", ""),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "total": true
        });
        self.request(
            "/api/mv/first",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
