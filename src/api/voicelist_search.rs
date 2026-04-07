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
            "keyword": query.get_or("keyword", ""),
            "scene": "normal",
            "limit": query.get_or("limit", "10"),
            "offset": query.get_or("offset", "30"),
            "e_r": true
        });
        self.request(
            "/api/search/voicelist/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
