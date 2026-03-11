use super::Query;
use crate::error::Result;
/// 用户绑定信息
/// 对应 Node.js module/user_binding.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户绑定信息
    /// 对应 /user/bindling
    pub async fn user_binding(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            &format!("/api/v1/user/bindings/{}", query.get_or("uid", "0")),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
