use super::Query;
use crate::error::Result;
/// 云随机播放
/// 对应 Node.js module/playmode_song_vector.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云随机播放
    /// 对应 /playmode/song/vector
    pub async fn playmode_song_vector(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "ids": query.get_or("ids", "")
        });
        self.request(
            "/api/playmode/song/vector/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
