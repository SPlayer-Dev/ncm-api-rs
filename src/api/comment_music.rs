use super::Query;
use crate::error::Result;
/// 歌曲评论
/// 对应 Node.js module/comment_music.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲评论
    /// 对应 /comment/music
    pub async fn comment_music(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0");
        let data = json!({
            "rid": id,
            "limit": query.get_or("limit", "20").parse::<i64>().unwrap_or(20),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "beforeTime": query.get_or("before", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            &format!("/api/v1/resource/comments/R_SO_4_{}", id),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
