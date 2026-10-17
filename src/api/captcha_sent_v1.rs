use super::Query;
use crate::error::Result;
/// 发送验证码 v1
/// 对应 Node.js module/captcha_sent_v1.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 发送验证码 v1
    /// 对应 /captcha/sent/v1
    pub async fn captcha_sent_v1(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "ctcode": query.get_or("ctcode", "86"),
            "secrete": "music_middleuser_pclogin",
            "cellphone": query.get_or("phone", ""),
            "scene": "0"
        });
        self.request(
            "/api/middle/captcha/sent/v1",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
