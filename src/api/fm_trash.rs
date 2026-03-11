use super::Query;
use crate::error::Result;
/// FM 垃圾桶
/// 对应 Node.js module/fm_trash.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// FM 垃圾桶（不再播放）
    /// 对应 /fm/trash
    pub async fn fm_trash(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "0"),
            "alg": "RT",
            "time": query.get_or("time", "25").parse::<i64>().unwrap_or(25)
        });
        self.request(
            "/api/radio/trash/add",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
