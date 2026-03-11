use super::Query;
use crate::error::Result;
/// 每日签到
/// 对应 Node.js module/daily_signin.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 每日签到
    /// 对应 /daily/signin
    pub async fn daily_signin(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type": query.get_or("type", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/point/dailyTask",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
