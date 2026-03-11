use super::Query;
use crate::error::Result;
/// 云贝今日签到信息
/// 对应 Node.js module/yunbei_today.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝今日签到信息
    /// 对应 /yunbei/today
    pub async fn yunbei_today(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/point/today/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
