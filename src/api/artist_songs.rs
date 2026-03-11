use super::Query;
use crate::error::Result;
/// 歌手歌曲
/// 对应 Node.js module/artist_songs.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手歌曲
    /// 对应 /artist/songs
    pub async fn artist_songs(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0"),
            "private_cloud": "true",
            "work_type": 1,
            "order": query.get_or("order", "hot"),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "limit": query.get_or("limit", "100").parse::<i64>().unwrap_or(100)
        });
        self.request(
            "/api/v1/artist/songs",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
