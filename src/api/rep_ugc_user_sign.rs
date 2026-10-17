use super::Query;
use crate::error::Result;
/// 云小编每日签到
/// 对应 Node.js module/rep_ugc_user_sign.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编每日签到
    /// 对应 /rep/ugc/user/sign
    pub async fn rep_ugc_user_sign(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/rep/ugc/user/sign",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
