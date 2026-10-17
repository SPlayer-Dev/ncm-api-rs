use super::Query;
use crate::error::Result;
/// 云小编领取一日会员
/// 对应 Node.js module/rep_ugc_user_collect-vip.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编领取一日会员
    /// 对应 /rep/ugc/user/collect-vip
    pub async fn rep_ugc_user_collect_vip(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "activityId": query.get_or("activityId", "5001")
        });
        self.request(
            "/api/rep/ugc/user/collect-vip",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
