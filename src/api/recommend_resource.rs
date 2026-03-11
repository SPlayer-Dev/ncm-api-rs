use super::Query;
use crate::error::Result;
/// 推荐歌单
/// 对应 Node.js module/recommend_resource.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 推荐歌单（需要登录）
    /// 对应 /recommend/resource
    pub async fn recommend_resource(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/v1/discovery/recommend/resource",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
