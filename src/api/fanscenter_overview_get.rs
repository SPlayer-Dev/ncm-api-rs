use super::Query;
use crate::error::Result;
/// 粉丝数量
/// 对应 Node.js module/fanscenter_overview_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 粉丝数量
    /// 对应 /fanscenter/overview/get
    pub async fn fanscenter_overview_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/fanscenter/overview/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
