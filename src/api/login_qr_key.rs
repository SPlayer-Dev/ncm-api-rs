use super::Query;
use crate::error::Result;
/// 二维码 key 生成
/// 对应 Node.js module/login_qr_key.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 二维码 key 生成
    /// 对应 /login/qr/key
    pub async fn login_qr_key(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type": 3
        });
        let res = self
            .request(
                "/api/login/qrcode/unikey",
                data,
                query.to_option(CryptoType::default()),
            )
            .await?;
        // 与 Node 版一致：包一层 data
        Ok(ApiResponse {
            status: 200,
            body: json!({ "data": res.body, "code": 200 }),
            cookie: res.cookie,
        })
    }
}
