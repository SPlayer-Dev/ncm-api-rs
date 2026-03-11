use super::Query;
use crate::error::Result;
/// 用户贡献条目、积分、云贝数量
/// 对应 Node.js module/ugc_user_devote.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户贡献条目、积分、云贝数量
    /// 对应 /ugc/user/devote
    pub async fn ugc_user_devote(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/rep/ugc/user/devote",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
