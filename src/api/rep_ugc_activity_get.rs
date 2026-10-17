use super::Query;
use crate::error::Result;
/// 云小编活动信息
/// 对应 Node.js module/rep_ugc_activity_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编活动信息
    /// 对应 /rep/ugc/activity/get
    pub async fn rep_ugc_activity_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/rep/ugc/activity/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
