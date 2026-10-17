use super::Query;
use crate::error::Result;
/// 举报评论
/// 对应 Node.js module/comment_report.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 举报评论
    /// 对应 /comment/report
    pub async fn comment_report(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({
            "threadId": format!("R_SO_4_{}", query.get_or("id", ""))
        });
        if let Some(v) = query.get("cid") {
            data["commentId"] = json!(v);
        }
        if let Some(v) = query.get("reason") {
            data["reason"] = json!(v);
        }
        self.request(
            "/api/report/reportcomment",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
