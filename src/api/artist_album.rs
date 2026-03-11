use super::Query;
use crate::error::Result;
/// 歌手专辑
/// 对应 Node.js module/artist_album.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手专辑
    /// 对应 /artist/album
    pub async fn artist_album(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request(
            &format!("/api/artist/albums/{}", query.get_or("id", "0")),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
