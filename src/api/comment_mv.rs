use super::Query;
use crate::error::Result;
/// MV 评论
/// 对应 Node.js module/comment_mv.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// MV 评论
    /// 对应 /comment/mv
    pub async fn comment_mv(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0");
        let data = json!({
            "rid": id,
            "limit": query.get_or("limit", "20").parse::<i64>().unwrap_or(20),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "beforeTime": query.get_or("before", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            &format!("/api/v1/resource/comments/R_MV_5_{}", id),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
