use super::Query;
use crate::error::Result;
/// 当前账号关注的用户/歌手
/// 对应 Node.js module/user_follow_mixed.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 当前账号关注的用户/歌手
    /// 对应 /user/follow/mixed
    pub async fn user_follow_mixed(&self, query: &Query) -> Result<ApiResponse> {
        let size = query.get_or("size", "30");
        let cursor = query.get_or("cursor", "0");
        let scene = query.get_or("scene", "0");
        let page = json!({
            "size": size,
            "cursor": cursor
        });
        let data = json!({
            "authority": "false",
            "page": page.to_string(),
            "scene": scene,
            "size": size,
            "sortType": "0"
        });
        self.request(
            "/api/user/follow/users/mixed/get/v2",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
