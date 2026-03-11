use super::Query;
use crate::error::Result;
/// 二维码生成
/// 对应 Node.js module/login_qr_create.js
///
/// 注意: Node.js 版本使用 qrcode 库生成二维码图片，Rust 版本仅返回 URL，
/// 不包含 qrimg 生成（需要调用方自行处理）
use crate::request::{ApiClient, ApiResponse};
use serde_json::json;

impl ApiClient {
    /// 二维码生成
    /// 对应 /login/qr/create
    pub async fn login_qr_create(&self, query: &Query) -> Result<ApiResponse> {
        let key = query.get("key").unwrap_or("");
        let url = format!("https://music.163.com/login?codekey={}", key);
        Ok(ApiResponse {
            status: 200,
            body: json!({
                "code": 200,
                "data": {
                    "qrurl": url,
                    "qrimg": ""
                }
            }),
            cookie: vec![],
        })
    }
}
