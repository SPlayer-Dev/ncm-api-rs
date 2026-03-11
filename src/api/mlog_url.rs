use super::Query;
use crate::error::Result;
/// mlog链接
/// 对应 Node.js module/mlog_url.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// mlog链接
    /// 对应 /mlog/url
    pub async fn mlog_url(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", ""),
            "resolution": query.get_or("res", "1080").parse::<i64>().unwrap_or(1080),
            "type": 1
        });
        self.request(
            "/api/mlog/detail/v1",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
