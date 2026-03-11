use super::Query;
use crate::error::Result;
/// 声音列表详情
/// 对应 Node.js module/voicelist_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 声音列表详情
    /// 对应 /voicelist/detail
    pub async fn voicelist_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "")
        });
        self.request(
            "/api/voice/workbench/voicelist/detail",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
