use super::Query;
use crate::error::Result;
/// 获取用户的创建歌单列表
/// 对应 Node.js module/user_playlist_create.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取用户的创建歌单列表
    /// 对应 /user/playlist/create
    pub async fn user_playlist_create(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "100"),
            "offset": query.get_or("offset", "0"),
            "userId": query.get("uid").unwrap_or(""),
            "isWebview": "true",
            "includeRedHeart": "true",
            "includeTop": "true"
        });
        self.request(
            "/api/user/playlist/create",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
