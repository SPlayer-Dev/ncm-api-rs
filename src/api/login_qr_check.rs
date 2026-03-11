use super::Query;
use crate::error::Result;
/// 二维码检测扫码状态
/// 对应 Node.js module/login_qr_check.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 二维码检测扫码状态
    /// 对应 /login/qr/check
    pub async fn login_qr_check(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "key": query.get_or("key", ""),
            "type": 3
        });
        self.request(
            "/api/login/qrcode/client/login",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
