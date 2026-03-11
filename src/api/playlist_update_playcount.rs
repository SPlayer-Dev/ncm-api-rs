use super::Query;
use crate::error::Result;
/// 歌单打卡
/// 对应 Node.js module/playlist_update_playcount.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单打卡
    /// 对应 /playlist/update/playcount
    pub async fn playlist_update_playcount(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "")
        });
        self.request(
            "/api/playlist/update/playcount",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
