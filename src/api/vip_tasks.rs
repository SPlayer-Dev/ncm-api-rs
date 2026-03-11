use super::Query;
use crate::error::Result;
/// 会员任务
/// 对应 Node.js module/vip_tasks.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 会员任务列表
    /// 对应 /vip/tasks
    pub async fn vip_tasks(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/vipnewcenter/app/level/task/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
