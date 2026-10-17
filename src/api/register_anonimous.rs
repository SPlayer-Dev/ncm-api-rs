use super::Query;
use crate::error::Result;
/// 匿名注册
/// 对应 Node.js module/register_anonimous.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use base64::{engine::general_purpose::STANDARD, Engine};
use md5::{Digest, Md5};
use serde_json::json;

const ID_XOR_KEY_1: &str = "3go8&$8*3*3h0k(2)2";

fn cloudmusic_dll_encode_id(some_id: &str) -> String {
    let xored: Vec<u8> = some_id
        .bytes()
        .enumerate()
        .map(|(i, b)| {
            let key_byte = ID_XOR_KEY_1.as_bytes()[i % ID_XOR_KEY_1.len()];
            b ^ key_byte
        })
        .collect();
    let digest = Md5::digest(&xored);
    STANDARD.encode(digest)
}

impl ApiClient {
    /// 匿名注册
    /// 对应 /register/anonimous
    pub async fn register_anonimous(&self, query: &Query) -> Result<ApiResponse> {
        // 使用客户端当前设备 ID：游客 token 与设备绑定，后续请求的 deviceId 需一致
        let device_id = self.device_id().to_string();
        let id_with_hash = format!("{} {}", device_id, cloudmusic_dll_encode_id(&device_id));
        let encoded_id = STANDARD.encode(id_with_hash.as_bytes());
        let data = json!({
            "username": encoded_id
        });
        let mut result = self
            .request(
                "/api/register/anonimous",
                data,
                query.to_option(CryptoType::Xeapi),
            )
            .await?;
        // 与 Node 版一致：把 set-cookie 拼接后放进 body.cookie
        if let Some(obj) = result.body.as_object_mut() {
            obj.insert("cookie".to_string(), json!(result.cookie.join(";")));
        }
        Ok(result)
    }
}
