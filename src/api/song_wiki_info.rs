use super::Query;
use crate::error::Result;
/// 歌曲百科
/// 对应 Node.js module/song_wiki_info.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲百科
    /// 对应 /song/wiki/info
    pub async fn song_wiki_info(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "extJson": json!({"states": {"playingResource": {"current": query.get_or("id", ""), "scene": "songWiki"}}}).to_string(),
            "positionCode": "songWikiMainPosition"
        });
        self.request(
            "/api/link/page/parent/relation/construct/info",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
