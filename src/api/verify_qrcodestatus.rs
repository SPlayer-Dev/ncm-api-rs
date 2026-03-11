use super::Query;
use crate::error::Result;
/// 验证二维码状态
/// 对应 Node.js module/verify_qrcodestatus.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 验证二维码状态
    /// 对应 /verify/qrcodestatus
    pub async fn verify_qrcodestatus(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "qrCode": query.get("qr").unwrap_or("")
        });
        self.request(
            "/api/frontrisk/verify/qrcodestatus",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
