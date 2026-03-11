use super::Query;
use crate::error::Result;
/// 发送验证码
/// 对应 Node.js module/captcha_sent.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 发送验证码
    /// 对应 /captcha/sent
    pub async fn captcha_sent(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "ctcode": query.get_or("ctcode", "86"),
            "secrete": "music_middleuser_pclogin",
            "cellphone": query.get("phone").unwrap_or("")
        });
        self.request(
            "/api/sms/captcha/sent",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
