use super::Query;
use crate::error::Result;
/// 评论点赞
/// 对应 Node.js module/comment_like.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 评论点赞/取消点赞
    /// 对应 /comment/like
    pub async fn comment_like(&self, query: &Query) -> Result<ApiResponse> {
        let t = query.get_or("t", "1");
        let path = if t == "1" { "like" } else { "unlike" };
        let resource_type = query.get_or("type", "0");
        let thread_id = crate::util::config::RESOURCE_TYPE_MAP
            .get(resource_type.as_str())
            .map(|prefix| format!("{}{}", prefix, query.get_or("id", "0")))
            .unwrap_or_default();
        let data = json!({
            "threadId": thread_id,
            "commentId": query.get_or("cid", "0")
        });
        self.request(
            &format!("/api/v1/comment/{}", path),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
