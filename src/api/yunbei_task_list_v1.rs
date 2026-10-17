use super::Query;
use crate::error::Result;
/// 云贝广告任务 - 查询今日任务状态
/// 对应 Node.js module/yunbei_task_list_v1.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝广告任务 - 查询今日任务状态
    /// 对应 /yunbei/task/list/v1
    pub async fn yunbei_task_list_v1(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/ad/power/yunbei/distribution/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
