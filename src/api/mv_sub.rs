use super::Query;
use crate::error::Result;
/// 收藏/取消收藏 MV
/// 对应 Node.js module/mv_sub.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 收藏/取消收藏 MV
    /// 对应 /mv/sub
    pub async fn mv_sub(&self, query: &Query) -> Result<ApiResponse> {
        let t = query.get_or("t", "1");
        let path = if t == "1" { "sub" } else { "unsub" };
        let mvid = query.get_or("mvid", "0");
        let data = json!({
            "mvId": mvid,
            "mvIds": format!(r#"["{}"]"#, mvid)
        });
        self.request(
            &format!("/api/mv/{}", path),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
