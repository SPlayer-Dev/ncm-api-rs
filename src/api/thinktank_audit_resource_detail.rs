use super::Query;
use crate::error::Result;
/// 云小编获取任务
/// 对应 Node.js module/thinktank_audit_resource_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编获取任务
    /// 对应 /thinktank/audit/resource/detail
    ///
    /// type: 1 曲风, 2 语种, 3 原唱, 4 情绪标签（默认 4）
    pub async fn thinktank_audit_resource_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type": query.get_or("type", "4")
        });
        self.request(
            "/api/thinktank/audit/resource/detail",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
