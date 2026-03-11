use super::Query;
use crate::error::Result;
/// 网易出品
/// 对应 Node.js module/mv_exclusive_rcmd.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 网易出品
    /// 对应 /mv/exclusive/rcmd
    pub async fn mv_exclusive_rcmd(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "offset": query.get_or("offset", "0"),
            "limit": query.get_or("limit", "30")
        });
        self.request(
            "/api/mv/exclusive/rcmd",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
