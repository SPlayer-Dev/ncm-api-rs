use super::Query;
use crate::error::Result;
/// 粉丝年龄比例
/// 对应 Node.js module/fanscenter_basicinfo_age_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 粉丝年龄比例
    /// 对应 /fanscenter/basicinfo/age/get
    pub async fn fanscenter_basicinfo_age_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/fanscenter/basicinfo/age/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
