use super::Query;
use crate::error::Result;
/// 用户动态
/// 对应 Node.js module/user_event.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户动态
    /// 对应 /user/event
    pub async fn user_event(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "getcounts": true,
            "time": query.get_or("lasttime", "-1").parse::<i64>().unwrap_or(-1),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "total": false
        });
        self.request(
            &format!("/api/event/get/{}", query.get_or("uid", "0")),
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
