use super::Query;
use crate::error::Result;
/// 搜索歌手
/// 对应 Node.js module/ugc_artist_search.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 搜索歌手
    /// 对应 /ugc/artist/search
    pub async fn ugc_artist_search(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "keyword": query.get_or("keyword", ""),
            "limit": query.get_or("limit", "40")
        });
        self.request(
            "/api/rep/ugc/artist/search",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
