use super::Query;
use crate::error::Result;
/// 关注/取消关注用户
/// 对应 Node.js module/follow.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 关注/取消关注用户
    /// 对应 /follow
    pub async fn follow(&self, query: &Query) -> Result<ApiResponse> {
        let t = query.get_or("t", "1");
        let path = if t == "1" { "follow" } else { "delfollow" };
        let data = json!({});
        self.request(
            &format!("/api/user/{}/{}", path, query.get_or("id", "0")),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
