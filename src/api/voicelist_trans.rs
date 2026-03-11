use super::Query;
use crate::error::Result;
/// 声音列表转移
/// 对应 Node.js module/voicelist_trans.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 声音列表转移
    /// 对应 /voicelist/trans
    pub async fn voicelist_trans(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "200"),
            "offset": query.get_or("offset", "0"),
            "radioId": query.get("radioId").unwrap_or(""),
            "programId": query.get_or("programId", "0"),
            "position": query.get_or("position", "1")
        });
        self.request(
            "/api/voice/workbench/radio/program/trans",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
