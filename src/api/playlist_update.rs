use super::Query;
use crate::error::Result;
/// 编辑歌单
/// 对应 Node.js module/playlist_update.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 编辑歌单（批量更新名称、描述、标签）
    /// 对应 /playlist/update
    pub async fn playlist_update(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0");
        let name = query.get_or("name", "");
        let desc = query.get_or("desc", "");
        let tags = query.get_or("tags", "");
        let data = json!({
            "/api/playlist/desc/update": format!(r#"{{"id":{},"desc":"{}"}}"#, id, desc),
            "/api/playlist/tags/update": format!(r#"{{"id":{},"tags":"{}"}}"#, id, tags),
            "/api/playlist/update/name": format!(r#"{{"id":{},"name":"{}"}}"#, id, name)
        });
        self.request("/api/batch", data, query.to_option(CryptoType::default()))
            .await
    }
}
