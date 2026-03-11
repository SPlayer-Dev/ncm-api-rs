use super::Query;
use crate::error::Result;
/// MV 排行榜
/// 对应 Node.js module/top_mv.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// MV 排行榜
    /// 对应 /top/mv
    pub async fn top_mv(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "area": query.get_or("area", ""),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request("/api/mv/toplist", data, query.to_option(CryptoType::Weapi))
            .await
    }
}
