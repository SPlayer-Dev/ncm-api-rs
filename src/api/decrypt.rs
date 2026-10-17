use super::Query;
use crate::crypto;
use crate::error::Result;
/// 通用解密（调试用）
/// 对应 Node.js module/decrypt.js
use crate::request::{ApiClient, ApiResponse};
use serde_json::{json, Value};

fn bad_request(message: String) -> ApiResponse {
    ApiResponse {
        status: 400,
        body: json!({ "code": 400, "message": message }),
        cookie: vec![],
    }
}

impl ApiClient {
    /// 通用解密
    /// 对应 /decrypt
    ///
    /// 参数：crypto（eapi/weapi/linuxapi/xeapi/api，默认 eapi）, data 或 hexString, isReq（默认 true）
    pub async fn decrypt(&self, query: &Query) -> Result<ApiResponse> {
        let crypto_type = query.get_or("crypto", "eapi");
        let data = query
            .get("data")
            .or_else(|| query.get("hexString"))
            .unwrap_or("");
        let is_req = query.get("isReq") != Some("false");
        if data.is_empty() {
            return Ok(bad_request("data is required".into()));
        }
        let pure_hex: String = data.chars().filter(|c| !c.is_whitespace()).collect();

        let result: std::result::Result<Value, String> = match (crypto_type.as_str(), is_req) {
            ("eapi", true) => crypto::eapi_req_decrypt(&pure_hex)
                .map(|(url, data)| json!({ "url": url, "data": data }))
                .ok_or_else(|| "invalid eapi request".to_string()),
            ("eapi", false) | ("weapi", false) => {
                crypto::eapi_res_decrypt(&pure_hex).ok_or_else(|| "invalid eapi response".to_string())
            }
            ("weapi", true) => {
                return Ok(bad_request(
                    "weapi 请求解密需要 RSA 私钥，暂不支持；仅支持 weapi 返回数据解密（e_r=true 时与 eapi 相同）".into(),
                ))
            }
            ("linuxapi", true) => crypto::linuxapi_req_decrypt(&pure_hex)
                .ok_or_else(|| "invalid linuxapi request".to_string()),
            ("linuxapi", false) | ("api", _) => serde_json::from_str(data).map_err(|e| e.to_string()),
            ("xeapi", true) => {
                return Ok(bad_request(
                    "xeapi 请求解密涉及 X25519 ECDH 密钥交换，流程复杂，暂不支持；仅支持 xeapi 返回数据解密".into(),
                ))
            }
            ("xeapi", false) => {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD
                    .decode(data)
                    .map_err(|e| e.to_string())
                    .and_then(|buf| crypto::xeapi_res_decrypt(&buf))
            }
            (other, _) => return Ok(bad_request(format!("未知加密方式: {}", other))),
        };

        Ok(match result {
            Ok(v) => ApiResponse {
                status: 200,
                body: json!({ "code": 200, "data": v }),
                cookie: vec![],
            },
            Err(e) => bad_request(format!("解密失败: {}", e)),
        })
    }
}
