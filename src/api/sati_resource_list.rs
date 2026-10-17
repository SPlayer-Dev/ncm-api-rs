use super::Query;
use crate::error::Result;
/// 助眠解压 - 标签下资源列表
/// 对应 Node.js module/sati_resource_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 助眠解压 - 标签下资源列表
    /// 对应 /sati/resource/list
    pub async fn sati_resource_list(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({
            "firstQuery": false
        });
        if let Some(v) = query.get("tag") {
            data["tag"] = json!(v);
        }
        self.request(
            "/api/voice/sati/resource/list",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
