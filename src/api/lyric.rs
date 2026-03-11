use super::Query;
use crate::error::Result;
/// 歌词
/// 对应 Node.js module/lyric.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取歌词
    /// 对应 /lyric
    pub async fn lyric(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0"),
            "tv": -1,
            "lv": -1,
            "rv": -1,
            "kv": -1,
            "_nmclfl": 1
        });
        self.request(
            "/api/song/lyric",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
