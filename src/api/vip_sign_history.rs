use super::Query;
use crate::error::Result;
/// 黑胶乐签打卡历史 / 状态查询
/// 对应 Node.js module/vip_sign_history.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 黑胶乐签打卡历史 / 状态查询
    /// 对应 /vip/sign/history
    ///
    /// type: 0 用户信息栏, 1 黑胶乐签
    pub async fn vip_sign_history(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type": query.get_or("type", "0")
        });
        self.request(
            "/api/vipnewcenter/app/minidesk/music/sign/pc",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
