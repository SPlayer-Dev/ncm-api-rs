use super::Query;
use crate::error::Result;
/// 歌手相关视频
/// 对应 Node.js module/artist_video.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手相关视频
    /// 对应 /artist/video
    pub async fn artist_video(&self, query: &Query) -> Result<ApiResponse> {
        let page = json!({
            "size": query.get_or("size", "10").parse::<i64>().unwrap_or(10),
            "cursor": query.get_or("cursor", "0").parse::<i64>().unwrap_or(0)
        });
        let data = json!({
            "artistId": query.get_or("id", "0"),
            "page": page.to_string(),
            "tab": 0,
            "order": query.get_or("order", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/mlog/artist/video",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
