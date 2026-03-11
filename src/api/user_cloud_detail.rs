use super::Query;
use crate::error::Result;
/// 用户云盘歌曲详情
/// 对应 Node.js module/user_cloud_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户云盘歌曲详情
    /// 对应 /user/cloud/detail
    pub async fn user_cloud_detail(&self, query: &Query) -> Result<ApiResponse> {
        let ids: Vec<String> = query
            .get_or("id", "")
            .replace(' ', "")
            .split(',')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let data = json!({
            "songIds": ids
        });
        self.request(
            "/api/v1/cloud/get/byids",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
