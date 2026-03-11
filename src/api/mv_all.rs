use super::Query;
use crate::error::Result;
/// MV 全部
/// 对应 Node.js module/mv_all.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 全部 MV
    /// 对应 /mv/all
    pub async fn mv_all(&self, query: &Query) -> Result<ApiResponse> {
        let tags = json!({
            "地区": query.get_or("area", "全部"),
            "类型": query.get_or("type", "全部"),
            "排序": query.get_or("order", "上升最快")
        });
        let data = json!({
            "tags": serde_json::to_string(&tags).unwrap_or_default(),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": "true",
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30)
        });
        self.request("/api/mv/all", data, query.to_option(CryptoType::default()))
            .await
    }
}
