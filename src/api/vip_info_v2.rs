use super::Query;
use crate::error::Result;
/// 获取 VIP 信息 v2
/// 对应 Node.js module/vip_info_v2.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取 VIP 信息 v2
    /// 对应 /vip/info/v2
    pub async fn vip_info_v2(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "userId": query.get_or("uid", "")
        });
        self.request(
            "/api/music-vip-membership/client/vip/info",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
