use super::Query;
use crate::error::Result;
/// 评论
/// 对应 Node.js module/msg_comments.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 评论
    /// 对应 /msg/comments
    pub async fn msg_comments(&self, query: &Query) -> Result<ApiResponse> {
        let uid = query.get_or("uid", "0");
        let data = json!({
            "beforeTime": query.get_or("before", "-1"),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "total": "true",
            "uid": uid
        });
        self.request(
            &format!("/api/v1/user/comments/{}", uid),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
