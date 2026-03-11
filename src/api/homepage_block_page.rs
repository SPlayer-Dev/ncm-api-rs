use super::Query;
use crate::error::Result;
/// 首页 Block Page
/// 对应 Node.js module/homepage_block_page.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 首页内容
    /// 对应 /homepage/block/page
    pub async fn homepage_block_page(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "refresh": query.get_or("refresh", "false") == "true",
            "cursor": query.get_or("cursor", "")
        });
        self.request(
            "/api/homepage/block/page",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
