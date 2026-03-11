use super::Query;
use crate::error::Result;
/// 云贝推歌历史记录
/// 对应 Node.js module/yunbei_rcmd_song_history.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝推歌历史记录
    /// 对应 /yunbei/rcmd/song/history
    pub async fn yunbei_rcmd_song_history(&self, query: &Query) -> Result<ApiResponse> {
        let page = json!({
            "size": query.get_or("size", "20").parse::<i64>().unwrap_or(20),
            "cursor": query.get_or("cursor", "")
        });
        let data = json!({
            "page": page.to_string()
        });
        self.request(
            "/api/yunbei/rcmd/song/history/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
