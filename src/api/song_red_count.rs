use super::Query;
use crate::error::Result;
/// 歌曲红心数量
/// 对应 Node.js module/song_red_count.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲红心数量
    /// 对应 /song/red/count
    pub async fn song_red_count(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "0"),
        });
        self.request(
            "/api/song/red/count",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
