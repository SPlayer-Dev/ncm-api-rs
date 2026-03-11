use super::Query;
use crate::error::Result;
/// 用户徽章
/// 对应 Node.js module/user_medal.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户徽章
    /// 对应 /user/medal
    pub async fn user_medal(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "uid": query.get("uid").unwrap_or("")
        });
        self.request(
            "/api/medal/user/page",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
