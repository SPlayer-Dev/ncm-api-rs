use super::Query;
use crate::error::Result;
/// 听歌足迹 - 歌曲播放排行 (Top20)
/// 对应 Node.js module/listen_data_song_play_rank.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 听歌足迹 - 歌曲播放排行 (Top20)
    /// 对应 /listen/data/song/play/rank
    ///
    /// type: week / month（默认 month）；endTime 不填为本周/月
    pub async fn listen_data_song_play_rank(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({
            "type": query.get_or("type", "month")
        });
        if let Some(v) = query.get("endTime") {
            data["endTime"] = json!(v);
        }
        self.request(
            "/api/content/activity/listen/data/song/play/rank",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
