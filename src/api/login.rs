use super::Query;
use crate::error::Result;
/// 邮箱登录
/// 对应 Node.js module/login.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use md5::{Digest, Md5};
use serde_json::json;

impl ApiClient {
    /// 邮箱登录
    /// 对应 /login
    ///
    /// 注意: Node.js 版本会对密码做 MD5 处理，Rust 版本需要传入 md5_password 或预先 MD5 后的 password
    pub async fn login(&self, query: &Query) -> Result<ApiResponse> {
        let password = if let Some(md5_pw) = query.get("md5_password") {
            md5_pw.to_string()
        } else {
            let pw = query.get("password").unwrap_or("");
            format!("{:x}", Md5::digest(pw.as_bytes()))
        };
        let data = json!({
            "type": "0",
            "https": "true",
            "username": query.get("email").unwrap_or(""),
            "password": password,
            "rememberLogin": "true"
        });
        let mut result = self
            .request("/api/w/login", data, query.to_option(CryptoType::default()))
            .await?;
        if result.body.get("code").and_then(|v| v.as_i64()) == Some(502) {
            result.status = 200;
            result.body = json!({
                "msg": "账号或密码错误",
                "code": 502,
                "message": "账号或密码错误"
            });
        }
        Ok(result)
    }
}
