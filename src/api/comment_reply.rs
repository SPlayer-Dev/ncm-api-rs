use super::Query;
use crate::error::Result;
/// 回复评论
/// 对应 Node.js module/comment_reply.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 回复评论
    /// 对应 /comment/reply
    pub async fn comment_reply(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "threadId": query.get_or("threadId", ""),
            "commentId": query.get_or("commentId", ""),
            "content": query.get_or("content", ""),
            "resourceType": "0",
            "resourceId": "0"
        });
        self.request(
            "/api/v1/resource/comments/reply",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
