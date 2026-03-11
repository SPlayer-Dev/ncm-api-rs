use super::Query;
use crate::error::Result;
/// 新版评论
/// 对应 Node.js module/comment_new.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 新版评论
    /// 对应 /comment/new
    pub async fn comment_new(&self, query: &Query) -> Result<ApiResponse> {
        let resource_type = query.get_or("type", "0");
        let id = query.get_or("id", "0");
        let sort_type = query.get_or("sortType", "99").parse::<i64>().unwrap_or(99);
        let page_no = query.get_or("pageNo", "1").parse::<i64>().unwrap_or(1);
        let page_size = query.get_or("pageSize", "20").parse::<i64>().unwrap_or(20);

        // 确定正确的 sortType（1 -> 99）
        let sort_type = if sort_type == 1 { 99 } else { sort_type };

        // 计算 cursor
        let cursor = match sort_type {
            99 => format!("{}", (page_no - 1) * page_size),
            2 => format!("normalHot#{}", (page_no - 1) * page_size),
            _ => query.get_or("cursor", "0"),
        };

        let thread_id = crate::util::config::RESOURCE_TYPE_MAP
            .get(resource_type.as_str())
            .map(|prefix| format!("{}{}", prefix, id))
            .unwrap_or_default();

        let data = json!({
            "rid": id,
            "threadId": thread_id,
            "pageNo": page_no,
            "showInner": query.get_or("showInner", "true"),
            "pageSize": page_size,
            "cursor": cursor,
            "sortType": sort_type
        });
        self.request(
            "/api/v2/resource/comments",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
