use super::Query;
use crate::error::Result;
/// 更换绑定手机
/// 对应 Node.js module/user_replacephone.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 更换绑定手机
    /// 对应 /user/replacephone
    pub async fn user_replacephone(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "phone": query.get("phone").unwrap_or(""),
            "captcha": query.get("captcha").unwrap_or(""),
            "oldcaptcha": query.get("oldcaptcha").unwrap_or(""),
            "countrycode": query.get_or("countrycode", "86")
        });
        self.request(
            "/api/user/replaceCellphone",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
