use super::Query;
use crate::error::Result;
/// 我喜欢的歌单
/// 对应 Node.js module/playlist_mylike.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 我喜欢的歌单
    /// 对应 /playlist/mylike
    pub async fn playlist_mylike(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "time": query.get_or("time", "-1"),
            "limit": query.get_or("limit", "12")
        });
        self.request(
            "/api/mlog/playlist/mylike/bytime/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
