use super::Query;
use crate::error::Result;
/// 助眠解压 - 收藏
/// 对应 Node.js module/sati_resource_sub.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 助眠解压 - 收藏
    /// 对应 /sati/resource/sub
    pub async fn sati_resource_sub(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({
            "cancel": query.get_or("cancel", "false")
        });
        if let Some(v) = query.get("id") {
            data["id"] = json!(v);
        }
        self.request(
            "/api/voice/sati/resource/sub",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
