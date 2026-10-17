use super::Query;
use crate::error::Result;
/// 发送评论
/// 对应 Node.js module/comment_add.js
use crate::request::{ApiClient, ApiResponse, CheckToken, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 发送评论
    /// 对应 /comment/add
    ///
    /// 参数：id 资源 id, type 资源类型, content 内容
    pub async fn comment_add(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "threadId": super::comment_delete::thread_id(query),
            "content": query.get_or("content", ""),
            "resourceType": "0",
            "expressionPicId": "-1",
            "bubbleId": "-1"
        });
        self.request(
            "/api/resource/comments/add",
            data,
            query.to_option_token(CryptoType::Xeapi, CheckToken::V3),
        )
        .await
    }
}
