use super::Query;
use crate::error::Result;
/// 动态
/// 对应 Node.js module/event.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取动态
    /// 对应 /event
    pub async fn event(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "pagesize": query.get_or("pagesize", "20").parse::<i64>().unwrap_or(20),
            "lasttime": query.get_or("lasttime", "-1").parse::<i64>().unwrap_or(-1)
        });
        self.request(
            "/api/v1/event/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
