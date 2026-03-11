use super::Query;
use crate::error::Result;
/// 歌手列表
/// 对应 Node.js module/artist_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手列表
    /// 对应 /artist/list
    pub async fn artist_list(&self, query: &Query) -> Result<ApiResponse> {
        let initial_str = query.get_or("initial", "");
        let initial = if initial_str.is_empty() {
            serde_json::Value::Null
        } else if let Ok(n) = initial_str.parse::<i64>() {
            json!(n)
        } else {
            let ch = initial_str.to_uppercase().chars().next().unwrap_or('A');
            json!(ch as i64)
        };
        let data = json!({
            "initial": initial,
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "total": true,
            "type": query.get_or("type", "1"),
            "area": query.get_or("area", "")
        });
        self.request(
            "/api/v1/artist/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
