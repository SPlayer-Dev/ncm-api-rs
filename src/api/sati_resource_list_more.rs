use super::Query;
use crate::error::Result;
/// 助眠解压 - 同类推荐
/// 对应 Node.js module/sati_resource_list_more.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 助眠解压 - 同类推荐
    /// 对应 /sati/resource/list/more
    pub async fn sati_resource_list_more(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({});
        if let Some(v) = query.get("id") {
            data["id"] = json!(v);
        }
        self.request(
            "/api/voice/sati/resource/list/more/v1",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
