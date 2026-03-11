use super::Query;
use crate::error::Result;
/// 粉丝性别比例
/// 对应 Node.js module/fanscenter_basicinfo_gender_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 粉丝性别比例
    /// 对应 /fanscenter/basicinfo/gender/get
    pub async fn fanscenter_basicinfo_gender_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/fanscenter/basicinfo/gender/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
