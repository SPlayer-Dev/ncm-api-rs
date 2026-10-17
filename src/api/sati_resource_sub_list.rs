use super::Query;
use crate::error::Result;
/// 助眠解压 - 收藏列表
/// 对应 Node.js module/sati_resource_sub_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 助眠解压 - 收藏列表
    /// 对应 /sati/resource/sub/list
    pub async fn sati_resource_sub_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/voice/sati/resource/sub/list",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
