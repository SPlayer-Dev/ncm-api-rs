use super::Query;
use crate::error::Result;
/// 更换手机
/// 对应 Node.js module/rebind.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 更换手机
    /// 对应 /rebind
    pub async fn rebind(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "captcha": query.get("captcha").unwrap_or(""),
            "phone": query.get("phone").unwrap_or(""),
            "oldcaptcha": query.get("oldcaptcha").unwrap_or(""),
            "ctcode": query.get_or("ctcode", "86")
        });
        self.request(
            "/api/user/replaceCellphone",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
