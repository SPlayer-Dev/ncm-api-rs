use super::Query;
use crate::error::Result;
/// 新版歌词
/// 对应 Node.js module/lyric_new.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取新版歌词（含逐字歌词）
    /// 对应 /lyric/new
    pub async fn lyric_new(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0"),
            "cp": false,
            "tv": 0,
            "lv": 0,
            "rv": 0,
            "kv": 0,
            "yv": 0,
            "ytv": 0,
            "yrv": 0
        });
        self.request(
            "/api/song/lyric/v1",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
