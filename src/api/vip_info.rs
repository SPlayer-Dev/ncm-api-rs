use super::Query;
use crate::error::Result;
/// 获取 VIP 信息
/// 对应 Node.js module/vip_info.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取 VIP 信息
    /// 对应 /vip/info
    pub async fn vip_info(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "userId": query.get_or("uid", "")
        });
        self.request(
            "/api/music-vip-membership/front/vip/info",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
