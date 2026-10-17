use super::Query;
use crate::error::Result;
/// 云小编领取任务积分
/// 对应 Node.js module/rep_ugc_activity_collect.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编领取任务积分
    /// 对应 /rep/ugc/activity/collect
    ///
    /// activityId 调用 /rep/ugc/activity/get 获取
    pub async fn rep_ugc_activity_collect(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "activityId": query.get_or("activityId", "5001")
        });
        self.request(
            "/api/rep/ugc/activity/collect",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
