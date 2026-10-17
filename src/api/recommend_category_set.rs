use super::Query;
use crate::error::Result;
/// 每日风格推荐场景配置设置
/// 对应 Node.js module/recommend_category_set.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 每日风格推荐场景配置设置
    /// 对应 /recommend/category/set
    ///
    /// 参数：tags 逗号分隔的标签 id, category 场景 id
    pub async fn recommend_category_set(&self, query: &Query) -> Result<ApiResponse> {
        let tag_ids: Vec<i64> = query
            .get_or("tags", "")
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let data = json!({
            "tags": json!({
                "tagIds": tag_ids,
                "categoryId": query.get_i64("category", 0)
            })
            .to_string()
        });
        self.request(
            "/api/homepage/daily/song/tag/save",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
