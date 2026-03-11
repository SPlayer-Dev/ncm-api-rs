use super::Query;
use crate::error::Result;
/// 用户是否互相关注
/// 对应 Node.js module/user_mutualfollow_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户是否互相关注
    /// 对应 /user/mutualfollow/get
    pub async fn user_mutualfollow_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "friendid": query.get("uid").unwrap_or("")
        });
        self.request(
            "/api/user/mutualfollow/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
