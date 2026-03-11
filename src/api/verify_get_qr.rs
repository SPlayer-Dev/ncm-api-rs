use super::Query;
use crate::error::Result;
/// 获取验证二维码
/// 对应 Node.js module/verify_getQr.js
///
/// 注意: Node.js 版本使用 qrcode 库生成二维码图片，Rust 版本仅返回 URL 和 qrCode，
/// 不包含 qrimg 生成（需要调用方自行处理）
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取验证二维码
    /// 对应 /verify/getQr
    pub async fn verify_get_qr(&self, query: &Query) -> Result<ApiResponse> {
        let params = json!({
            "event_id": query.get("evid").unwrap_or(""),
            "sign": query.get("sign").unwrap_or("")
        });
        let data = json!({
            "verifyConfigId": query.get("vid").unwrap_or(""),
            "verifyType": query.get("type").unwrap_or(""),
            "token": query.get("token").unwrap_or(""),
            "params": params.to_string(),
            "size": 150
        });
        let res = self
            .request(
                "/api/frontrisk/verify/getqrcode",
                data,
                query.to_option(CryptoType::Weapi),
            )
            .await?;

        let qr_code = res
            .body
            .get("data")
            .and_then(|d| d.get("qrCode"))
            .and_then(|q| q.as_str())
            .unwrap_or("");

        let token = query.get("token").unwrap_or("");
        let vid = query.get("vid").unwrap_or("");
        let verify_type = query.get("type").unwrap_or("");
        let evid = query.get("evid").unwrap_or("");
        let sign = query.get("sign").unwrap_or("");
        let params_str = json!({"event_id": evid, "sign": sign}).to_string();

        let qrurl = format!(
            "https://st.music.163.com/encrypt-pages?qrCode={}&verifyToken={}&verifyId={}&verifyType={}&params={}",
            qr_code, token, vid, verify_type, params_str
        );

        Ok(ApiResponse {
            status: 200,
            body: json!({
                "code": 200,
                "data": {
                    "qrCode": qr_code,
                    "qrurl": qrurl,
                    "qrimg": ""
                }
            }),
            cookie: vec![],
        })
    }
}
