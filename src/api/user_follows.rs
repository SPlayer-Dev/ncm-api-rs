use super::Query;
use crate::error::Result;
/// 用户关注列表
/// 对应 Node.js module/user_follows.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户关注列表
    /// 对应 /user/follows
    pub async fn user_follows(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "order": true
        });
        self.request(
            &format!("/api/user/getfollows/{}", query.get_or("uid", "0")),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
