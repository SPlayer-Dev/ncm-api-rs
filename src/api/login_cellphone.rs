use super::Query;
use crate::error::Result;
/// 手机号登录
/// 对应 Node.js module/login_cellphone.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use md5::{Digest, Md5};
use serde_json::{json, Value};

impl ApiClient {
    /// 手机号登录
    /// 对应 /login/cellphone
    pub async fn login_cellphone(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({
            "type": "1",
            "https": "true",
            "phone": query.get_or("phone", ""),
            "countrycode": query.get_or("countrycode", "86"),
            "remember": "true",
            "secureCaptcha": query.get_or("sca", "")
        });

        // 与 Node 版一致：有验证码时只传 captcha，否则传 MD5 后的 password
        if let Some(captcha) = query.get("captcha") {
            data["captcha"] = Value::String(captcha.to_string());
        } else {
            let password = if let Some(md5_pwd) = query.get("md5_password") {
                md5_pwd.to_string()
            } else {
                hex::encode(Md5::digest(query.get_or("password", "").as_bytes()))
            };
            data["password"] = Value::String(password);
        }

        let res = self
            .request(
                "/api/w/login/cellphone",
                data,
                query.to_option(CryptoType::Weapi),
            )
            .await?;
        // 与 Node 版一致：avatarImgId_str 改名为 avatarImgIdStr，并在 body 中返回拼接后的 cookie
        let mut body: Value = serde_json::from_str(
            &res.body
                .to_string()
                .replace("avatarImgId_str", "avatarImgIdStr"),
        )?;
        if let Some(obj) = body.as_object_mut() {
            obj.insert("cookie".to_string(), Value::String(res.cookie.join(";")));
        }
        Ok(ApiResponse {
            status: res.status,
            body,
            cookie: res.cookie,
        })
    }
}
