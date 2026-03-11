use super::Query;
use crate::error::Result;
/// 智能播放列表
/// 对应 Node.js module/playmode_intelligence_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 智能播放列表
    /// 对应 /playmode/intelligence/list
    pub async fn playmode_intelligence_list(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0");
        let data = json!({
            "songId": id,
            "type": "fromPlayOne",
            "playlistId": query.get_or("pid", "0"),
            "startMusicId": query.get_or("sid", &id),
            "count": query.get_or("count", "1").parse::<i64>().unwrap_or(1)
        });
        self.request(
            "/api/playmode/intelligence/list",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
