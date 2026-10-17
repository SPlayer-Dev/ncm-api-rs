use super::Query;
use crate::error::Result;
/// 获取指定维度音乐排行榜详情
/// 对应 Node.js module/chart_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取指定维度音乐排行榜详情
    /// 对应 /chart/detail
    pub async fn chart_detail(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({});
        if let Some(v) = query.get("chartCode") {
            data["chartCode"] = json!(v);
        }
        if let Some(v) = query.get("targetId") {
            data["targetId"] = json!(v);
        }
        if let Some(v) = query.get("targetType") {
            data["targetType"] = json!(v);
        }
        self.request("/api/chart/detail", data, query.to_option(CryptoType::Eapi))
            .await
    }
}
