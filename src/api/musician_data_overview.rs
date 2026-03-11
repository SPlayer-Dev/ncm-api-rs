use super::Query;
use crate::error::Result;
/// 音乐人数据概况
/// 对应 Node.js module/musician_data_overview.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 音乐人数据概况
    /// 对应 /musician/data/overview
    pub async fn musician_data_overview(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/creator/musician/statistic/data/overview/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
