use super::Query;
use crate::error::Result;
/// 评论抱一抱列表
/// 对应 Node.js module/comment_hug_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 评论抱一抱列表
    /// 对应 /comment/hug/list
    pub async fn comment_hug_list(&self, query: &Query) -> Result<ApiResponse> {
        let resource_type = query.get_or("type", "0");
        let thread_id = crate::util::config::RESOURCE_TYPE_MAP
            .get(resource_type.as_str())
            .map(|prefix| format!("{}{}", prefix, query.get_or("sid", "0")))
            .unwrap_or_default();
        let data = json!({
            "targetUserId": query.get_or("uid", "0"),
            "commentId": query.get_or("cid", "0"),
            "cursor": query.get_or("cursor", "-1"),
            "threadId": thread_id,
            "pageNo": query.get_or("page", "1").parse::<i64>().unwrap_or(1),
            "idCursor": query.get_or("idCursor", "-1").parse::<i64>().unwrap_or(-1),
            "pageSize": query.get_or("pageSize", "100").parse::<i64>().unwrap_or(100)
        });
        self.request(
            "/api/v2/resource/comments/hug/list",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
