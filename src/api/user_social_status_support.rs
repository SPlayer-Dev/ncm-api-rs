use super::Query;
use crate::error::Result;
/// 用户状态 - 支持设置的状态
/// 对应 Node.js module/user_social_status_support.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户状态 - 支持设置的状态
    /// 对应 /user/social/status/support
    pub async fn user_social_status_support(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/social/user/status/support",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
