use super::Query;
use crate::error::Result;
/// 歌单动态信息
/// 对应 Node.js module/playlist_detail_dynamic.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单动态信息
    /// 对应 /playlist/detail/dynamic
    pub async fn playlist_detail_dynamic(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", ""),
            "n": 100000,
            "s": query.get_or("s", "8").parse::<i64>().unwrap_or(8)
        });
        self.request(
            "/api/playlist/detail/dynamic",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
