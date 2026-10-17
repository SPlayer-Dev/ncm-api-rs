use super::Query;
use crate::error::Result;
/// 助眠解压 - 标签列表
/// 对应 Node.js module/sati_tag_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 助眠解压 - 标签列表
    /// 对应 /sati/tag/list
    pub async fn sati_tag_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/voice/sati/tag/list",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
