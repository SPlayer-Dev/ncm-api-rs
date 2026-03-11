use super::Query;
use crate::error::Result;
/// 声音详情
/// 对应 Node.js module/voice_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 声音详情
    /// 对应 /voice/detail
    pub async fn voice_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "")
        });
        self.request(
            "/api/voice/workbench/voice/detail",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
