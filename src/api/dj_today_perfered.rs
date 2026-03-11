use super::Query;
use crate::error::Result;
/// 电台今日优选
/// 对应 Node.js module/dj_today_perfered.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台今日优选
    /// 对应 /dj/today/perfered
    pub async fn dj_today_perfered(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "page": query.get_or("page", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/djradio/home/today/perfered",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
