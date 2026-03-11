use super::Query;
use crate::error::Result;
/// 粉丝省份比例
/// 对应 Node.js module/fanscenter_basicinfo_province_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 粉丝省份比例
    /// 对应 /fanscenter/basicinfo/province/get
    pub async fn fanscenter_basicinfo_province_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/fanscenter/basicinfo/province/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
