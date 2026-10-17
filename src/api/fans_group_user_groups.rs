use super::Query;
use crate::error::Result;
/// 当前登录用户加入的全部歌手乐迷团列表
/// 对应 Node.js module/fans_group_user_groups.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 当前登录用户加入的全部歌手乐迷团列表
    /// 对应 /fans/group/user/groups
    pub async fn fans_group_user_groups(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/social/fansgroup/bff/user/groups/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
