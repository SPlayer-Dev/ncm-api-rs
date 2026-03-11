use super::Query;
use crate::error::Result;
/// 付费电台
/// 对应 Node.js module/dj_paygift.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 付费电台
    /// 对应 /dj/paygift
    pub async fn dj_paygift(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "_nmclfl": 1
        });
        self.request(
            "/api/djradio/home/paygift/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
