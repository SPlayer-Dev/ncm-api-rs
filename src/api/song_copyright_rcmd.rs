use super::Query;
use crate::error::Result;
/// 灰色歌曲的其他版本推荐
/// 对应 Node.js module/song_copyright_rcmd.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 灰色歌曲的其他版本推荐
    /// 对应 /song/copyright/rcmd
    pub async fn song_copyright_rcmd(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songid": query.get("songid").or_else(|| query.get("id")).unwrap_or("").to_string()
        });
        self.request(
            "/api/song/copyright/rcmd",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
