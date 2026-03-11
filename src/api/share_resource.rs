use super::Query;
use crate::error::Result;
/// 分享歌曲到动态
/// 对应 Node.js module/share_resource.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 分享歌曲到动态
    /// 对应 /share/resource
    pub async fn share_resource(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type": query.get_or("type", "song"),
            "msg": query.get_or("msg", ""),
            "id": query.get_or("id", "")
        });
        self.request(
            "/api/share/friends/resource",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
