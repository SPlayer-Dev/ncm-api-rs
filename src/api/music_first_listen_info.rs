use super::Query;
use crate::error::Result;
/// 回忆坐标
/// 对应 Node.js module/music_first_listen_info.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 回忆坐标
    /// 对应 /music/first/listen/info
    pub async fn music_first_listen_info(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "")
        });
        self.request(
            "/api/content/activity/music/first/listen/info",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
