use super::Query;
use crate::error::Result;
/// 排行榜列表
/// 对应 Node.js module/toplist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 排行榜列表
    /// 对应 /toplist
    pub async fn toplist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request("/api/toplist", data, query.to_option(CryptoType::default()))
            .await
    }
}
