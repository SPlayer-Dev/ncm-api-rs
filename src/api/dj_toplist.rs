use super::Query;
use crate::error::Result;
/// 电台排行榜
/// 对应 Node.js module/dj_toplist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台排行榜
    /// 对应 /dj/toplist
    pub async fn dj_toplist(&self, query: &Query) -> Result<ApiResponse> {
        let type_val = query.get_or("type", "new");
        let type_id = match type_val.as_str() {
            "hot" => 1,
            _ => 0,
        };
        let data = json!({
            "limit": query.get_or("limit", "100").parse::<i64>().unwrap_or(100),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "type": type_id
        });
        self.request(
            "/api/djradio/toplist",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
