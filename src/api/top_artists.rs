use super::Query;
use crate::error::Result;
/// 热门歌手
/// 对应 Node.js module/top_artists.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 热门歌手
    /// 对应 /top/artists
    pub async fn top_artists(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "50").parse::<i64>().unwrap_or(50),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request("/api/artist/top", data, query.to_option(CryptoType::Weapi))
            .await
    }
}
