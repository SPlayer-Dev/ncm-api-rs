use super::Query;
use crate::error::Result;
/// 云贝待完成任务
/// 对应 Node.js module/yunbei_tasks_todo.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝待完成任务列表
    /// 对应 /yunbei/tasks/todo
    pub async fn yunbei_tasks_todo(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/usertool/task/todo/query",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
