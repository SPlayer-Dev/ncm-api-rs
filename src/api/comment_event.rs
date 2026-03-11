use super::Query;
use crate::error::Result;
/// 获取动态评论
/// 对应 Node.js module/comment_event.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取动态评论
    /// 对应 /comment/event
    pub async fn comment_event(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "20"),
            "offset": query.get_or("offset", "0"),
            "beforeTime": query.get_or("before", "0")
        });
        self.request(
            &format!("/api/v1/resource/comments/{}", query.get_or("threadId", "")),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
