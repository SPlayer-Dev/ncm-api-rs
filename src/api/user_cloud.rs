use super::Query;
use crate::error::Result;
/// 用户云盘
/// 对应 Node.js module/user_cloud.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户云盘数据
    /// 对应 /user/cloud
    pub async fn user_cloud(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/v1/cloud/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
