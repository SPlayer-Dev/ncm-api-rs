use super::Query;
use crate::error::Result;
/// 用户所处乐迷团详情
/// 对应 Node.js module/fans_group_user_group_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户所处乐迷团详情
    /// 对应 /fans/group/user/group/detail
    pub async fn fans_group_user_group_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "groupId": query.get("groupId").or_else(|| query.get("id")).unwrap_or("").to_string()
        });
        self.request(
            "/api/social/fansgroup/bff/user/group/detail/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
