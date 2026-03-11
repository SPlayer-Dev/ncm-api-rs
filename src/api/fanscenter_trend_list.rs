use super::Query;
use crate::error::Result;
/// 粉丝来源
/// 对应 Node.js module/fanscenter_trend_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 粉丝来源
    /// 对应 /fanscenter/trend/list
    pub async fn fanscenter_trend_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "startTime": query.get_or("startTime", ""),
            "endTime": query.get_or("endTime", ""),
            "type": query.get_or("type", "0")
        });
        self.request(
            "/api/fanscenter/trend/list",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
