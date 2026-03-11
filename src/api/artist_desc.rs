use super::Query;
use crate::error::Result;
/// 歌手介绍
/// 对应 Node.js module/artist_desc.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手介绍
    /// 对应 /artist/desc
    pub async fn artist_desc(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0")
        });
        self.request(
            "/api/artist/introduction",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
