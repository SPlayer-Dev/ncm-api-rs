use super::Query;
use crate::error::{NcmError, Result};
/// 云小编提交任务
/// 对应 Node.js module/thinktank_audit_resource_update.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编提交任务
    /// 对应 /thinktank/audit/resource/update
    ///
    /// judgement: 1 同意, 2 否决, 3 跳过
    pub async fn thinktank_audit_resource_update(&self, query: &Query) -> Result<ApiResponse> {
        if query.get("taskId").is_none_or(str::is_empty) {
            return Err(NcmError::InvalidParam("参数不足: taskId".into()));
        }
        if query.get("judgement").is_none_or(str::is_empty) {
            return Err(NcmError::InvalidParam("参数不足: judgement".into()));
        }
        let data = json!({
            "type": query.get_or("type", "4"),
            "taskId": query.get_or("taskId", ""),
            "judgement": query.get_or("judgement", "")
        });
        self.request(
            "/api/thinktank/audit/resource/update",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
