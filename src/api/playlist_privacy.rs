use super::Query;
use crate::error::Result;
/// 公开隐私歌单
/// 对应 Node.js module/playlist_privacy.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 公开隐私歌单
    /// 对应 /playlist/privacy
    pub async fn playlist_privacy(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", ""),
            "privacy": 0
        });
        self.request(
            "/api/playlist/update/privacy",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
