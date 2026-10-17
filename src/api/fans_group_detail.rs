use super::Query;
use crate::error::Result;
/// 乐迷团详情
/// 对应 Node.js module/fans_group_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 乐迷团详情
    /// 对应 /fans/group/detail
    pub async fn fans_group_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "groupId": query.get("groupId").or_else(|| query.get("id")).unwrap_or("").to_string(),
            "scene": query.get_or("scene", "")
        });
        self.request(
            "/api/social/fansgroup/bff/detail/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
