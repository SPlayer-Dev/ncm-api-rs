use super::Query;
use crate::error::Result;
/// 云小编获取用户详情
/// 对应 Node.js module/rep_ugc_user_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编获取用户详情
    /// 对应 /rep/ugc/user/get
    pub async fn rep_ugc_user_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/rep/ugc/user/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
