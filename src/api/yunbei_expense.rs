use super::Query;
use crate::error::Result;
/// 云贝支出
/// 对应 Node.js module/yunbei_expense.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝支出记录
    /// 对应 /yunbei/expense
    pub async fn yunbei_expense(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "10").parse::<i64>().unwrap_or(10),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/point/expense",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
