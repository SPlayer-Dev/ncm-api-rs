use super::Query;
use crate::error::Result;
/// 添加歌曲到歌单
/// 对应 Node.js module/playlist_track_add.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 添加歌曲到歌单
    /// 对应 /playlist/track/add
    pub async fn playlist_track_add(&self, query: &Query) -> Result<ApiResponse> {
        let tracks: Vec<serde_json::Value> = query
            .get_or("ids", "")
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .map(|id| json!({"type": 3, "id": id}))
            .collect();
        let data = json!({
            "id": query.get_or("pid", "0"),
            "tracks": serde_json::to_string(&tracks).unwrap_or_default()
        });
        self.request(
            "/api/playlist/track/add",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
