use super::Query;
use crate::error::Result;
/// 电台评论
/// 对应 Node.js module/comment_dj.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台评论
    /// 对应 /comment/dj
    pub async fn comment_dj(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0");
        let data = json!({
            "rid": id,
            "limit": query.get_or("limit", "20"),
            "offset": query.get_or("offset", "0"),
            "beforeTime": query.get_or("before", "0")
        });
        self.request(
            &format!("/api/v1/resource/comments/A_DJ_1_{}", id),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
