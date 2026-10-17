use super::Query;
use crate::error::Result;
/// 云贝广告任务 - 完成任务领取云贝
/// 对应 Node.js module/yunbei_task_finish_v1.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝广告任务 - 完成任务领取云贝
    /// 对应 /yunbei/task/finish/v1
    ///
    /// yunbeiAmount: 单次可得云贝（默认 150），单日上限 10 次
    pub async fn yunbei_task_finish_v1(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "yunbeiAmount": query.get_or("yunbeiAmount", "150")
        });
        self.request(
            "/api/ad/power/yunbei/distribution/create",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
