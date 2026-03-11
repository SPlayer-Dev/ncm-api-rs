use super::Query;
use crate::error::Result;
/// 用户状态 - 相同状态的用户
/// 对应 Node.js module/user_social_status_rcmd.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户状态 - 相同状态的用户
    /// 对应 /user/social/status/rcmd
    pub async fn user_social_status_rcmd(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/social/user/status/rcmd",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
