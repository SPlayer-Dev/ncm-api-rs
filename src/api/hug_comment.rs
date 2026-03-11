use super::Query;
use crate::error::Result;
/// 抱一抱评论
/// 对应 Node.js module/hug_comment.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 抱一抱评论
    /// 对应 /hug/comment
    pub async fn hug_comment(&self, query: &Query) -> Result<ApiResponse> {
        let resource_type = query.get_or("type", "0");
        let type_prefix = crate::util::config::RESOURCE_TYPE_MAP
            .get(resource_type.as_str())
            .copied()
            .unwrap_or("R_SO_4_");
        let thread_id = format!("{}{}", type_prefix, query.get_or("sid", "0"));
        let data = json!({
            "targetUserId": query.get_or("uid", "0"),
            "commentId": query.get_or("cid", "0"),
            "threadId": thread_id
        });
        self.request(
            "/api/v2/resource/comments/hug/listener",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
