use super::Query;
use crate::error::Result;
/// 用户详情
/// 对应 Node.js module/user_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户详情
    /// 对应 /user/detail
    pub async fn user_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            &format!("/api/v1/user/detail/{}", query.get_or("uid", "0")),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
