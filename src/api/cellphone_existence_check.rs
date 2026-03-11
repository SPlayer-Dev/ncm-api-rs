use super::Query;
use crate::error::Result;
/// 检测手机号码是否已注册
/// 对应 Node.js module/cellphone_existence_check.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 检测手机号码是否已注册
    /// 对应 /cellphone/existence/check
    pub async fn cellphone_existence_check(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "cellphone": query.get("phone").unwrap_or(""),
            "countrycode": query.get("countrycode").unwrap_or("")
        });
        self.request(
            "/api/cellphone/existence/check",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
