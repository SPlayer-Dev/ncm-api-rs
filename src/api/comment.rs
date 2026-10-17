use super::Query;
use crate::error::Result;
/// 发送/删除/回复评论
/// 对应 Node.js module/comment.js
use crate::request::{ApiClient, ApiResponse, CheckToken, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 发送/删除/回复评论
    /// 对应 /comment
    /// t: 1=发送, 0=删除, 2=回复
    pub async fn comment(&self, query: &Query) -> Result<ApiResponse> {
        let t = query.get_or("t", "1");
        let action = match t.as_str() {
            "1" => "add",
            "0" => "delete",
            "2" => "reply",
            _ => "add",
        };
        let resource_type = query.get_or("type", "0");
        let thread_id = match crate::util::config::RESOURCE_TYPE_MAP.get(resource_type.as_str()) {
            // 动态评论直接使用 threadId
            Some(&"A_EV_2_") => query.get_or("threadId", ""),
            Some(prefix) => format!("{}{}", prefix, query.get_or("id", "0")),
            None => String::new(),
        };
        let mut data = json!({
            "threadId": thread_id
        });
        match action {
            "add" => {
                data["content"] = json!(query.get_or("content", ""));
            }
            "delete" => {
                data["commentId"] = json!(query.get_or("commentId", "0"));
            }
            "reply" => {
                data["commentId"] = json!(query.get_or("commentId", "0"));
                data["content"] = json!(query.get_or("content", ""));
            }
            _ => {}
        }
        self.request(
            &format!("/api/resource/comments/{}", action),
            data,
            query.to_option_token(CryptoType::Eapi, CheckToken::Static),
        )
        .await
    }
}
