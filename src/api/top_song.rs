use super::Query;
use crate::error::Result;
/// 新歌速递
/// 对应 Node.js module/top_song.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 新歌速递
    /// 对应 /top/song
    pub async fn top_song(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "areaId": query.get_or("type", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request(
            "/api/v1/discovery/new/songs",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
