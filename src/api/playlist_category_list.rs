use super::Query;
use crate::error::Result;
/// 歌单分类列表
/// 对应 Node.js module/playlist_category_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单分类列表
    /// 对应 /playlist/catlist
    pub async fn playlist_category_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "cat": query.get_or("cat", "全部"),
            "limit": query.get_or("limit", "24").parse::<i64>().unwrap_or(24),
            "newStyle": true
        });
        self.request(
            "/api/playlist/category/list",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
