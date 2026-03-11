use super::Query;
use crate::error::Result;
/// MV简要百科信息
/// 对应 Node.js module/ugc_mv_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// MV简要百科信息
    /// 对应 /ugc/mv/get
    pub async fn ugc_mv_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "mvId": query.get_or("id", "")
        });
        self.request(
            "/api/rep/ugc/mv/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
