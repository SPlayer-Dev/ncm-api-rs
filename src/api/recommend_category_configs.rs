use super::Query;
use crate::error::Result;
/// 每日风格推荐场景配置列表
/// 对应 Node.js module/recommend_category_configs.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 每日风格推荐场景配置列表
    /// 对应 /recommend/category/configs
    pub async fn recommend_category_configs(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/homepage/daily/song/config/get",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
