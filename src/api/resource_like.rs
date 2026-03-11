use super::Query;
use crate::error::Result;
/// 点赞与取消点赞资源
/// 对应 Node.js module/resource_like.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 点赞与取消点赞资源
    /// 对应 /resource/like
    pub async fn resource_like(&self, query: &Query) -> Result<ApiResponse> {
        let t = if query.get_or("t", "0") == "1" {
            "like"
        } else {
            "unlike"
        };

        let resource_type_prefix = match query.get_or("type", "0").as_str() {
            "0" => "R_SO_4_",
            "1" => "R_MV_5_",
            "2" => "A_PL_0_",
            "3" => "R_AL_3_",
            "4" => "A_DJ_1_",
            "5" => "R_VI_62_",
            "6" => "A_EV_2_",
            "7" => "A_DR_14_",
            _ => "R_SO_4_",
        };

        let thread_id = if resource_type_prefix == "A_EV_2_" {
            query.get_or("threadId", "").to_string()
        } else {
            format!("{}{}", resource_type_prefix, query.get_or("id", "0"))
        };

        let data = json!({
            "threadId": thread_id
        });
        self.request(
            &format!("/api/resource/{}", t),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
