use super::Query;
use crate::error::Result;
/// 歌单评论
/// 对应 Node.js module/comment_playlist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单评论
    /// 对应 /comment/playlist
    pub async fn comment_playlist(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0");
        let data = json!({
            "rid": id,
            "limit": query.get_or("limit", "20").parse::<i64>().unwrap_or(20),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "beforeTime": query.get_or("before", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            &format!("/api/v1/resource/comments/A_PL_0_{}", id),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
