use super::Query;
use crate::error::Result;
/// 电台热门
/// 对应 Node.js module/dj_hot.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 热门电台
    /// 对应 /dj/hot
    pub async fn dj_hot(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/djradio/hot/v1",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
