use super::Query;
use crate::error::Result;
/// 专辑动态信息
/// 对应 Node.js module/album_detail_dynamic.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 专辑动态信息
    /// 对应 /album/detail/dynamic
    pub async fn album_detail_dynamic(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0")
        });
        self.request(
            "/api/album/detail/dynamic",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
