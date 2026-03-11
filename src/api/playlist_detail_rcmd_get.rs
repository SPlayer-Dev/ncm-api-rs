use super::Query;
use crate::error::Result;
/// 相关歌单推荐
/// 对应 Node.js module/playlist_detail_rcmd_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 相关歌单推荐
    /// 对应 /playlist/detail/rcmd/get
    pub async fn playlist_detail_rcmd_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "scene": "playlist_head",
            "playlistId": query.get_or("id", ""),
            "newStyle": "true"
        });
        self.request(
            "/api/playlist/detail/rcmd/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
