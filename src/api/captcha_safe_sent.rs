use super::Query;
use crate::error::Result;
/// 发送安全验证码
/// 对应 Node.js module/captcha_safe_sent.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 发送安全验证码
    /// 对应 /captcha/safe/sent
    pub async fn captcha_safe_sent(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "ctcode": query.get_or("ctcode", "86")
        });
        self.request(
            "/api/sms/captcha/safe/sent",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
