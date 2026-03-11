use super::Query;
use crate::error::Result;
/// 曲风列表
/// 对应 Node.js module/style_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 曲风列表
    /// 对应 /style/list
    pub async fn style_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/tag/list/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
