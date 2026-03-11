use super::Query;
use crate::error::Result;
/// 声音列表
/// 对应 Node.js module/voicelist_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 声音列表
    /// 对应 /voicelist/list
    pub async fn voicelist_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "200"),
            "offset": query.get_or("offset", "0"),
            "voiceListId": query.get_or("voiceListId", "")
        });
        self.request(
            "/api/voice/workbench/voices/by/voicelist",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
