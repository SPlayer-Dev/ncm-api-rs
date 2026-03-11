use super::Query;
use crate::error::Result;
/// 云贝签到
/// 对应 Node.js module/yunbei_sign.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝签到
    /// 对应 /yunbei/sign
    pub async fn yunbei_sign(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/pointmall/user/sign",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
