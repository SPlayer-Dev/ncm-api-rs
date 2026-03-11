use super::Query;
use crate::error::Result;
/// 收藏/取消收藏歌曲到歌单
/// 对应 Node.js module/playlist_tracks.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 收藏/删除歌曲到歌单
    /// 对应 /playlist/tracks
    pub async fn playlist_tracks(&self, query: &Query) -> Result<ApiResponse> {
        let tracks: Vec<String> = query
            .get_or("tracks", "")
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let data = json!({
            "op": query.get_or("op", "add"),
            "pid": query.get_or("pid", "0"),
            "trackIds": serde_json::to_string(&tracks).unwrap_or_default(),
            "imme": "true"
        });
        self.request(
            "/api/playlist/manipulate/tracks",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
