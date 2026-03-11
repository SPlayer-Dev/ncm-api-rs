use super::Query;
use crate::error::Result;
/// 付费精品
/// 对应 Node.js module/dj_toplist_pay.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 付费精品
    /// 对应 /dj/toplist/pay
    pub async fn dj_toplist_pay(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "100").parse::<i64>().unwrap_or(100)
        });
        self.request(
            "/api/djradio/toplist/pay",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
