use super::Query;
use crate::error::Result;
/// 删除评论
/// 对应 Node.js module/comment_delete.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 删除评论
    /// 对应 /comment/delete
    pub async fn comment_delete(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "commentId": query.get_or("commentId", ""),
            "threadId": query.get_or("threadId", "")
        });
        self.request(
            "/api/resource/comments/delete",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
