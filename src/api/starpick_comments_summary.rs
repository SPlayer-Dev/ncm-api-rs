use super::Query;
use crate::error::Result;
/// 云村星评馆 - 简要评论列表
/// 对应 Node.js module/starpick_comments_summary.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云村星评馆 - 简要评论列表
    /// 对应 /starpick/comments/summary
    pub async fn starpick_comments_summary(&self, query: &Query) -> Result<ApiResponse> {
        let cursor = json!({
            "offset": 0,
            "blockCodeOrderList": ["HOMEPAGE_BLOCK_NEW_HOT_COMMENT"],
            "refresh": true
        });
        let data = json!({
            "cursor": cursor.to_string()
        });
        self.request(
            "/api/homepage/block/page",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
