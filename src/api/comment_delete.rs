use super::Query;
use crate::error::Result;
/// 删除评论
/// 对应 Node.js module/comment_delete.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

/// threadId = 资源类型前缀 + id（type 缺省为 0 歌曲）
pub(super) fn thread_id(query: &Query) -> String {
    let prefix = crate::util::config::RESOURCE_TYPE_MAP
        .get(query.get_or("type", "0").as_str())
        .copied()
        .unwrap_or("R_SO_4_");
    format!("{}{}", prefix, query.get_or("id", ""))
}

impl ApiClient {
    /// 删除评论
    /// 对应 /comment/delete
    ///
    /// 参数：id 资源 id, type 资源类型, cid 评论 id（兼容旧参数名 commentId）
    pub async fn comment_delete(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "commentId": query.get("cid").or_else(|| query.get("commentId")).unwrap_or(""),
            "threadId": thread_id(query)
        });
        self.request(
            "/api/resource/comments/delete",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
