use super::Query;
use crate::error::Result;
/// 账号云豆数
/// 对应 Node.js module/musician_cloudbean.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 账号云豆数
    /// 对应 /musician/cloudbean
    pub async fn musician_cloudbean(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/cloudbean/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
