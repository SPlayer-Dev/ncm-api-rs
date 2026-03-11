use super::Query;
use crate::error::Result;
/// 获取音乐人任务(新)
/// 对应 Node.js module/musician_tasks_new.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取音乐人任务(新)
    /// 对应 /musician/tasks/new
    pub async fn musician_tasks_new(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/nmusician/workbench/mission/stage/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
