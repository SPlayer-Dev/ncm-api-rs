use super::Query;
use crate::error::Result;
/// 助眠解压 - 特定时间场景推荐资源
/// 对应 Node.js module/sati_timescene_resources_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 助眠解压 - 特定时间场景推荐资源
    /// 对应 /sati/timescene/resources/get
    pub async fn sati_timescene_resources_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "firstQuery": false
        });
        self.request(
            "/api/voice/sati/timescene/resources/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
