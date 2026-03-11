use super::Query;
use crate::error::Result;
/// 歌单导入 - 任务状态
/// 对应 Node.js module/playlist_import_task_status.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单导入 - 任务状态
    /// 对应 /playlist/import/task/status
    pub async fn playlist_import_task_status(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "");
        let data = json!({
            "taskIds": format!("[\"{}\"]", id)
        });
        self.request(
            "/api/playlist/import/task/status/v2",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
