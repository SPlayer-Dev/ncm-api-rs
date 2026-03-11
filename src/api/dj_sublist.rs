use super::Query;
use crate::error::Result;
/// 电台订阅列表
/// 对应 Node.js module/dj_sublist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 已订阅电台列表
    /// 对应 /dj/sublist
    pub async fn dj_sublist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request(
            "/api/djradio/get/subed",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
