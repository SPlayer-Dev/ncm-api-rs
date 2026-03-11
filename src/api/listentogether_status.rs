use super::Query;
use crate::error::Result;
/// 一起听状态
/// 对应 Node.js module/listentogether_status.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 一起听状态
    /// 对应 /listentogether/status
    pub async fn listentogether_status(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/listen/together/status/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
