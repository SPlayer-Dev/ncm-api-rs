use super::Query;
use crate::error::Result;
/// 会员成长值领取记录
/// 对应 Node.js module/vip_growthpoint_details.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 会员成长值领取记录
    /// 对应 /vip/growthpoint/details
    pub async fn vip_growthpoint_details(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "20").parse::<i64>().unwrap_or(20),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/vipnewcenter/app/level/growth/details",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
