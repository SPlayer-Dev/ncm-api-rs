use super::Query;
use crate::error::Result;
/// 最近播放的视频歌单
/// 对应 Node.js module/playlist_video_recent.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 最近播放的视频歌单
    /// 对应 /playlist/video/recent
    pub async fn playlist_video_recent(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/playlist/video/recent",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
