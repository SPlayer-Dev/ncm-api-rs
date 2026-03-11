use super::Query;
use crate::error::Result;
/// 通知
/// 对应 Node.js module/msg_notices.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 通知
    /// 对应 /msg/notices
    pub async fn msg_notices(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "time": query.get_or("lasttime", "-1").parse::<i64>().unwrap_or(-1)
        });
        self.request("/api/msg/notices", data, query.to_option(CryptoType::Weapi))
            .await
    }
}
