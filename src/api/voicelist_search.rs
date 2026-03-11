use super::Query;
use crate::error::Result;
/// 声音列表搜索
/// 对应 Node.js module/voicelist_search.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 声音列表搜索
    /// 对应 /voicelist/search
    pub async fn voicelist_search(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "fee": "-1",
            "limit": query.get_or("limit", "200"),
            "offset": query.get_or("offset", "0"),
            "podcastName": query.get_or("podcastName", "")
        });
        self.request(
            "/api/voice/workbench/voicelist/search",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
