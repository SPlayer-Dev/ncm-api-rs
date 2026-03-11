use super::Query;
use crate::error::Result;
/// 用户贡献内容
/// 对应 Node.js module/ugc_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户贡献内容
    /// 对应 /ugc/detail
    pub async fn ugc_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "auditStatus": query.get_or("auditStatus", ""),
            "limit": query.get_or("limit", "10"),
            "offset": query.get_or("offset", "0"),
            "order": query.get_or("order", "desc"),
            "sortBy": query.get_or("sortBy", "createTime"),
            "type": query.get_or("type", "1")
        });
        self.request(
            "/api/rep/ugc/detail",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
