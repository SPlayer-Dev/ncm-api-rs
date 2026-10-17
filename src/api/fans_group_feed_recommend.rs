use super::Query;
use crate::error::Result;
/// 乐迷团推荐笔记
/// 对应 Node.js module/fans_group_feed_recommend.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 乐迷团推荐笔记
    /// 对应 /fans/group/feed/recommend
    pub async fn fans_group_feed_recommend(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "artistSelf": query.get_or("artistSelf", "0"),
            "fansGroupId": query.get("fansGroupId").or_else(|| query.get("groupId")).unwrap_or("").to_string(),
            "cursor": query.get_or("cursor", "0"),
            "size": query.get_or("size", "10")
        });
        self.request(
            "/api/fans/group/feed/recommend/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
