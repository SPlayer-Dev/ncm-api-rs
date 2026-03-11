use super::Query;
use crate::error::Result;
/// 喜欢音乐
/// 对应 Node.js module/like.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 喜欢/取消喜欢音乐
    /// 对应 /like
    pub async fn like(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "alg": "itembased",
            "trackId": query.get_or("id", "0"),
            "like": query.get_or("like", "true") == "true",
            "time": "3"
        });
        self.request("/api/radio/like", data, query.to_option(CryptoType::Weapi))
            .await
    }
}
