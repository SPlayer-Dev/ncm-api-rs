use super::Query;
use crate::error::Result;
/// 会员任务 - 新版
/// 对应 Node.js module/vip_tasks_v1.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 会员任务 - 新版
    /// 对应 /vip/tasks/v1
    pub async fn vip_tasks_v1(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({
            "taskType": "app_vip_task_center"
        });
        if let Some(v) = query.get("id") {
            data["userId"] = json!(v);
        }
        self.request(
            "/api/middle/vip/mission/user/progress/list",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
