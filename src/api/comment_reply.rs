use super::Query;
use crate::error::Result;
/// 回复评论
/// 对应 Node.js module/comment_reply.js
use crate::request::{ApiClient, ApiResponse, CheckToken, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 回复评论
    /// 对应 /comment/reply
    ///
    /// 参数：id 资源 id, type 资源类型, cid 被回复评论 id, content 内容
    pub async fn comment_reply(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "threadId": super::comment_delete::thread_id(query),
            "commentId": query.get_or("cid", ""),
            "content": query.get_or("content", ""),
            "resourceType": "0"
        });
        self.request(
            "/api/v1/resource/comments/reply",
            data,
            query.to_option_token(CryptoType::Xeapi, CheckToken::V3),
        )
        .await
    }
}
