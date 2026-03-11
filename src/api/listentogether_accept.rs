use super::Query;
use crate::error::Result;
/// 一起听 接受邀请
/// 对应 Node.js module/listentogether_accept.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 一起听 接受邀请
    /// 对应 /listentogether/accept
    pub async fn listentogether_accept(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "refer": "inbox_invite",
            "roomId": query.get_or("roomId", ""),
            "inviterId": query.get_or("inviterId", "")
        });
        self.request(
            "/api/listen/together/play/invitation/accept",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
