use super::Query;
use crate::error::Result;
/// 音乐百科基础信息
/// 对应 Node.js module/song_wiki_summary.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 音乐百科基础信息
    /// 对应 /song/wiki/summary
    pub async fn song_wiki_summary(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "0"),
        });
        self.request(
            "/api/song/play/about/block/page",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
