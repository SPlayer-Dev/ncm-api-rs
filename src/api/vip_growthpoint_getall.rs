use super::Query;
use crate::error::Result;
/// 一键领取所有会员成长值
/// 对应 Node.js module/vip_growthpoint_getall.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 一键领取所有会员成长值
    /// 对应 /vip/growthpoint/getall
    pub async fn vip_growthpoint_getall(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/vipnewcenter/app/level/task/reward/getall",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
