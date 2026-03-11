use super::Query;
/// EAPI 解密
/// 对应 Node.js module/eapi_decrypt.js
use crate::crypto::{eapi_req_decrypt, eapi_res_decrypt};
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse};
use serde_json::json;

impl ApiClient {
    /// EAPI 解密
    /// 对应 /eapi/decrypt
    ///
    /// 参数：
    /// - hexString: 要解密的十六进制字符串
    /// - isReq: 是否为请求解密（默认 true），设为 "false" 则为响应解密
    pub fn eapi_decrypt(&self, query: &Query) -> Result<ApiResponse> {
        let hex_string = match query.get("hexString") {
            Some(s) => s,
            None => {
                return Ok(ApiResponse {
                    status: 400,
                    body: json!({
                        "code": 400,
                        "message": "hex string is required"
                    }),
                    cookie: vec![],
                });
            }
        };

        let is_req = query.get_or("isReq", "true") != "false";
        let pure_hex = hex_string.replace(' ', "");

        let data = if is_req {
            match eapi_req_decrypt(&pure_hex) {
                Some((url, body)) => json!({ "url": url, "body": body }),
                None => json!(null),
            }
        } else {
            match eapi_res_decrypt(&pure_hex) {
                Some(val) => val,
                None => json!(null),
            }
        };

        Ok(ApiResponse {
            status: 200,
            body: json!({
                "code": 200,
                "data": data
            }),
            cookie: vec![],
        })
    }
}
