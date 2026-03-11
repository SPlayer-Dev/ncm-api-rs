use super::Query;
use crate::error::Result;
/// 乐谱预览
/// 对应 Node.js module/sheet_preview.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 乐谱预览
    /// 对应 /sheet/preview
    pub async fn sheet_preview(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "")
        });
        self.request(
            "/api/music/sheet/preview/info",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
