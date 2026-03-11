use super::Query;
use crate::error::Result;
/// 类别热门电台
/// 对应 Node.js module/dj_radio_hot.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 类别热门电台
    /// 对应 /dj/radio/hot
    pub async fn dj_radio_hot(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "cateId": query.get_or("cateId", "0"),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0)
        });
        self.request("/api/djradio/hot", data, query.to_option(CryptoType::Weapi))
            .await
    }
}
