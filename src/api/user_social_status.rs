use super::Query;
use crate::error::Result;
/// 用户状态
/// 对应 Node.js module/user_social_status.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户状态
    /// 对应 /user/social/status
    pub async fn user_social_status(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "visitorId": query.get("uid").unwrap_or("")
        });
        self.request(
            "/api/social/user/status",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
