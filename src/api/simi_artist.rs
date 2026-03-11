use super::Query;
use crate::error::Result;
/// 相似歌手
/// 对应 Node.js module/simi_artist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 相似歌手
    /// 对应 /simi/artist
    pub async fn simi_artist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "artistid": query.get_or("id", "0")
        });
        self.request(
            "/api/discovery/simiArtist",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
