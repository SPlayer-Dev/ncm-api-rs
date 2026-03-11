use super::Query;
use crate::error::Result;
/// 设置
/// 对应 Node.js module/setting.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 设置
    /// 对应 /setting
    pub async fn setting(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/user/setting",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
