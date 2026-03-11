use super::Query;
use crate::error::Result;
/// 用户电台节目
/// 对应 Node.js module/user_dj.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户电台节目
    /// 对应 /user/dj
    pub async fn user_dj(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "30"),
            "offset": query.get_or("offset", "0")
        });
        self.request(
            &format!("/api/dj/program/{}", query.get_or("uid", "0")),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
