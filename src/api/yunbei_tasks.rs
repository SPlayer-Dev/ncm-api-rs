use super::Query;
use crate::error::Result;
/// 云贝所有任务
/// 对应 Node.js module/yunbei_tasks.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝所有任务列表
    /// 对应 /yunbei/tasks
    pub async fn yunbei_tasks(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/usertool/task/list/all",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
