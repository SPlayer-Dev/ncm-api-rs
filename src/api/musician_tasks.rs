use super::Query;
use crate::error::Result;
/// 获取音乐人任务
/// 对应 Node.js module/musician_tasks.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取音乐人任务
    /// 对应 /musician/tasks
    pub async fn musician_tasks(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/nmusician/workbench/mission/cycle/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
