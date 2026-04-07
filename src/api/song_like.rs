use super::Query;
use crate::error::Result;
/// 喜欢歌曲
/// 对应 Node.js module/song_like.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 喜欢歌曲
    /// 对应 /song/like
    pub async fn song_like(&self, query: &Query) -> Result<ApiResponse> {
        let like = query.get_or("like", "true") != "false";
        let data = json!({
            "trackId": query.get_or("id", ""),
            "userid": query.get_or("uid", ""),
            "like": like
        });
        self.request(
            "/api/song/like",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
