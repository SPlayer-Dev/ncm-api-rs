use super::Query;
use crate::error::Result;
/// 歌手 MV
/// 对应 Node.js module/artist_mv.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手 MV
    /// 对应 /artist/mv
    pub async fn artist_mv(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "artistId": query.get_or("id", "0"),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request("/api/artist/mvs", data, query.to_option(CryptoType::Weapi))
            .await
    }
}
