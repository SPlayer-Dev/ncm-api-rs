use super::Query;
use crate::error::Result;
/// 数字专辑&数字单曲-榜单
/// 对应 Node.js module/album_songsaleboard.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 数字专辑&数字单曲-榜单
    /// 对应 /album/songsaleboard
    pub async fn album_songsaleboard(&self, query: &Query) -> Result<ApiResponse> {
        let album_type = query.get_or("albumType", "0");
        let sale_type = query.get_or("type", "daily");

        let mut data = json!({
            "albumType": album_type.parse::<i64>().unwrap_or(0)
        });

        if sale_type == "year" {
            data["year"] = json!(query.get_or("year", ""));
        }

        let url = format!("/api/feealbum/songsaleboard/{}/type", sale_type);
        self.request(&url, data, query.to_option(CryptoType::Weapi))
            .await
    }
}
