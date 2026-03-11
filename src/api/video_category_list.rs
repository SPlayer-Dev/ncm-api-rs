use super::Query;
use crate::error::Result;
/// 视频分类列表
/// 对应 Node.js module/video_category_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 视频分类列表
    /// 对应 /video/category/list
    pub async fn video_category_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": "true",
            "limit": query.get_or("limit", "99").parse::<i64>().unwrap_or(99)
        });
        self.request(
            "/api/cloudvideo/category/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
