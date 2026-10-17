use super::Query;
use crate::error::Result;
/// 云小编查询会员任务状态
/// 对应 Node.js module/rep_ugc_user_vip.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编查询会员任务状态
    /// 对应 /rep/ugc/user/vip
    ///
    /// data.status: 10/20 可领取 1 日黑胶会员，30 已领取
    pub async fn rep_ugc_user_vip(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/rep/ugc/user/vip",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
