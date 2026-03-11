use super::Query;
use crate::error::Result;
/// 订阅/取消订阅电台
/// 对应 Node.js module/dj_sub.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 订阅/取消订阅电台
    /// 对应 /dj/sub
    pub async fn dj_sub(&self, query: &Query) -> Result<ApiResponse> {
        let t = query.get_or("t", "1");
        let path = if t == "1" { "sub" } else { "unsub" };
        let data = json!({
            "id": query.get_or("rid", "0")
        });
        self.request(
            &format!("/api/djradio/{}", path),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
