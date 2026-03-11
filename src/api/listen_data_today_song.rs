use super::Query;
use crate::error::Result;
/// 听歌足迹 - 今日收听
/// 对应 Node.js module/listen_data_today_song.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 听歌足迹 - 今日收听
    /// 对应 /listen/data/today/song
    pub async fn listen_data_today_song(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/content/activity/listen/data/today/song/play/rank",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
