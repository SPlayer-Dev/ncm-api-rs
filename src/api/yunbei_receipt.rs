use super::Query;
use crate::error::Result;
/// 云贝收入
/// 对应 Node.js module/yunbei_receipt.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝收入记录
    /// 对应 /yunbei/receipt
    pub async fn yunbei_receipt(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "10").parse::<i64>().unwrap_or(10),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/point/receipt",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
