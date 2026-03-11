use super::Query;
use crate::error::Result;
/// 声音搜索
/// 对应 Node.js module/voicelist_list_search.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 声音搜索
    /// 对应 /voicelist/list/search
    pub async fn voicelist_list_search(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "200"),
            "offset": query.get_or("offset", "0"),
            "name": query.get("name").unwrap_or(""),
            "displayStatus": query.get("displayStatus").unwrap_or(""),
            "type": query.get("type").unwrap_or(""),
            "voiceFeeType": query.get("voiceFeeType").unwrap_or(""),
            "radioId": query.get_or("voiceListId", "")
        });
        self.request(
            "/api/voice/workbench/voice/list",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
