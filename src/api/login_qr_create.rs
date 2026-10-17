use super::Query;
use crate::error::Result;
/// 二维码生成
/// 对应 Node.js module/login_qr_create.js
use crate::request::{ApiClient, ApiResponse};
use crate::util::cookie::cookie_to_json;
use serde_json::json;

/// 生成二维码 SVG 的 data URL
fn qr_data_url(text: &str) -> String {
    use base64::Engine;
    let svg = qrcode::QrCode::new(text.as_bytes())
        .map(|code| {
            code.render::<qrcode::render::svg::Color>()
                .min_dimensions(200, 200)
                .build()
        })
        .unwrap_or_default();
    format!(
        "data:image/svg+xml;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(svg)
    )
}

impl ApiClient {
    /// 二维码生成
    /// 对应 /login/qr/create
    ///
    /// 参数：key（/login/qr/key 返回的 unikey）, platform（web 时附带 chainId，网易云 App 扫码需要）, qrimg（为真时返回二维码图片 data URL）
    pub async fn login_qr_create(&self, query: &Query) -> Result<ApiResponse> {
        let mut url = format!(
            "https://music.163.com/login?codekey={}",
            query.get_or("key", "")
        );
        if query.get_or("platform", "pc") == "web" {
            let device_id = cookie_to_json(query.cookie.as_deref().unwrap_or(""))
                .remove("sDeviceId")
                .unwrap_or_else(|| format!("unknown-{}", rand::random::<u32>() % 1_000_000));
            url.push_str(&format!(
                "&chainId=v1_{}_web_login_{}",
                device_id,
                chrono::Utc::now().timestamp_millis()
            ));
        }
        let qrimg = if query
            .get("qrimg")
            .is_some_and(|v| !v.is_empty() && v != "false")
        {
            qr_data_url(&url)
        } else {
            String::new()
        };
        Ok(ApiResponse {
            status: 200,
            body: json!({
                "code": 200,
                "data": { "qrurl": url, "qrimg": qrimg }
            }),
            cookie: vec![],
        })
    }
}
