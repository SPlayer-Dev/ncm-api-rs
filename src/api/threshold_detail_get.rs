use super::Query;
use crate::error::Result;
/// 获取达人达标信息
/// 对应 Node.js module/threshold_detail_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取达人达标信息
    /// 对应 /threshold/detail/get
    pub async fn threshold_detail_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/influencer/web/apply/threshold/detail/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
