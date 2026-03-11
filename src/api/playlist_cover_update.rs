use super::Query;
use crate::error::Result;
/// 歌单封面更新
/// 对应 Node.js module/playlist_cover_update.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单封面更新
    /// 对应 /playlist/cover/update
    pub async fn playlist_cover_update(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", ""),
            "coverImgId": query.get_or("coverImgId", "")
        });
        self.request(
            "/api/playlist/cover/update",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
