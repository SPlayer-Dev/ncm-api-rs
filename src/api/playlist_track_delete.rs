use super::Query;
use crate::error::Result;
/// 从歌单中删除歌曲
/// 对应 Node.js module/playlist_track_delete.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 从歌单中删除歌曲
    /// 对应 /playlist/track/delete
    pub async fn playlist_track_delete(&self, query: &Query) -> Result<ApiResponse> {
        let tracks: Vec<serde_json::Value> = query
            .get_or("ids", "")
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .map(|id| json!({"type": 3, "id": id}))
            .collect();
        let data = json!({
            "id": query.get_or("id", "0"),
            "tracks": serde_json::to_string(&tracks).unwrap_or_default()
        });
        self.request(
            "/api/playlist/track/delete",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
