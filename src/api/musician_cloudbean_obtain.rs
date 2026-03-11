use super::Query;
use crate::error::Result;
/// 领取云豆
/// 对应 Node.js module/musician_cloudbean_obtain.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 领取云豆
    /// 对应 /musician/cloudbean/obtain
    pub async fn musician_cloudbean_obtain(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "userMissionId": query.get_or("id", ""),
            "period": query.get_or("period", "")
        });
        self.request(
            "/api/nmusician/workbench/mission/reward/obtain/new",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
