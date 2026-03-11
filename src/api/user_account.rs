use super::Query;
use crate::error::Result;
/// 用户账号信息
/// 对应 Node.js module/user_account.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户账号信息
    /// 对应 /user/account
    pub async fn user_account(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/nuser/account/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
