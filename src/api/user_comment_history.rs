use super::Query;
use crate::error::Result;
/// 用户评论历史
/// 对应 Node.js module/user_comment_history.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户评论历史
    /// 对应 /user/comment/history
    pub async fn user_comment_history(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "compose_reminder": "true",
            "compose_hot_comment": "true",
            "limit": query.get_or("limit", "10"),
            "user_id": query.get("uid").unwrap_or(""),
            "time": query.get_or("time", "0")
        });
        self.request(
            "/api/comment/user/comment/history",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
