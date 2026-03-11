use super::Query;
use crate::error::Result;
/// 注册账号
/// 对应 Node.js module/register_cellphone.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use md5::{Digest, Md5};
use serde_json::json;

impl ApiClient {
    /// 注册账号
    /// 对应 /register/cellphone
    ///
    /// 注意: Node.js 版本会对密码做 MD5 处理
    pub async fn register_cellphone(&self, query: &Query) -> Result<ApiResponse> {
        let password = query.get("password").unwrap_or("");
        let md5_password = format!("{:x}", Md5::digest(password.as_bytes()));
        let data = json!({
            "captcha": query.get("captcha").unwrap_or(""),
            "phone": query.get("phone").unwrap_or(""),
            "password": md5_password,
            "nickname": query.get("nickname").unwrap_or(""),
            "countrycode": query.get_or("countrycode", "86"),
            "force": "false"
        });
        self.request(
            "/api/w/register/cellphone",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
