use super::Query;
use crate::error::Result;
/// 获取用户的收藏歌单列表
/// 对应 Node.js module/user_playlist_collect.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取用户的收藏歌单列表
    /// 对应 /user/playlist/collect
    pub async fn user_playlist_collect(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "100"),
            "offset": query.get_or("offset", "0"),
            "userId": query.get("uid").unwrap_or(""),
            "isWebview": "true",
            "includeRedHeart": "true",
            "includeTop": "true"
        });
        self.request(
            "/api/user/playlist/collect",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
