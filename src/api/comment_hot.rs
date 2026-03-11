use super::Query;
use crate::error::Result;
/// 热门评论
/// 对应 Node.js module/comment_hot.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 热门评论
    /// 对应 /comment/hot
    pub async fn comment_hot(&self, query: &Query) -> Result<ApiResponse> {
        let resource_type = query.get_or("type", "0");
        let id = query.get_or("id", "0");
        let thread_id = crate::util::config::RESOURCE_TYPE_MAP
            .get(resource_type.as_str())
            .map(|prefix| format!("{}{}", prefix, id))
            .unwrap_or_default();
        let data = json!({
            "rid": id,
            "limit": query.get_or("limit", "20").parse::<i64>().unwrap_or(20),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "beforeTime": query.get_or("before", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            &format!("/api/v1/resource/hotcomments/{}", thread_id),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
