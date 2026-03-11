use super::Query;
use crate::error::Result;
/// 独家放送列表
/// 对应 Node.js module/personalized_privatecontent_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 独家放送列表
    /// 对应 /personalized/privatecontent/list
    pub async fn personalized_privatecontent_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": "true",
            "limit": query.get_or("limit", "60").parse::<i64>().unwrap_or(60)
        });
        self.request(
            "/api/v2/privatecontent/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
