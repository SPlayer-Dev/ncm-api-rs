use super::Query;
use crate::error::Result;
/// 用户粉丝列表
/// 对应 Node.js module/user_followeds.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户粉丝列表
    /// 对应 /user/followeds
    pub async fn user_followeds(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "userId": query.get_or("uid", "0"),
            "time": "0",
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "getcounts": "true"
        });
        self.request(
            &format!("/api/user/getfolloweds/{}", query.get_or("uid", "0")),
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
