use super::Query;
use crate::error::Result;
/// 独家放送
/// 对应 Node.js module/personalized_privatecontent.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 独家放送
    /// 对应 /personalized/privatecontent
    pub async fn personalized_privatecontent(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/personalized/privatecontent",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
