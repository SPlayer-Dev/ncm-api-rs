use super::Query;
use crate::error::Result;
/// 电台最热主播榜
/// 对应 Node.js module/dj_toplist_popular.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台最热主播榜
    /// 对应 /dj/toplist/popular
    pub async fn dj_toplist_popular(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "100").parse::<i64>().unwrap_or(100)
        });
        self.request(
            "/api/dj/toplist/popular",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
