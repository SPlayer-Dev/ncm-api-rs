use super::Query;
use crate::error::Result;
/// 本地歌曲匹配音乐信息
/// 对应 Node.js module/search_match.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 本地歌曲匹配音乐信息
    /// 对应 /search/match
    pub async fn search_match(&self, query: &Query) -> Result<ApiResponse> {
        let songs = json!([{
            "title": query.get_or("title", ""),
            "album": query.get_or("album", ""),
            "artist": query.get_or("artist", ""),
            "duration": query.get_or("duration", "0").parse::<i64>().unwrap_or(0),
            "persistId": query.get_or("md5", ""),
        }]);
        let data = json!({
            "songs": songs.to_string(),
        });
        self.request(
            "/api/search/match/new",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
