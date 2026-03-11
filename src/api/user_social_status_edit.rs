use super::Query;
use crate::error::Result;
/// 用户状态 - 编辑
/// 对应 Node.js module/user_social_status_edit.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户状态 - 编辑
    /// 对应 /user/social/status/edit
    pub async fn user_social_status_edit(&self, query: &Query) -> Result<ApiResponse> {
        let content = json!({
            "type": query.get("type").unwrap_or(""),
            "iconUrl": query.get("iconUrl").unwrap_or(""),
            "content": query.get("content").unwrap_or(""),
            "actionUrl": query.get("actionUrl").unwrap_or("")
        });
        let data = json!({
            "content": content.to_string()
        });
        self.request(
            "/api/social/user/status/edit",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
