use super::Query;
use crate::crypto::ncbl_encrypt;
use crate::error::Result;
/// 听歌打卡 - NCBL 加密版（仿桌面客户端 PLV/PLD 上报）
/// 对应 Node.js module/scrobble_v1.js + util/ncbl.js
use crate::request::{ApiClient, ApiResponse};
use crate::util::config::CL_DOMAIN3;
use crate::util::cookie::cookie_to_json;
use rand::RngExt;
use serde_json::{json, Value};

/// 从 cookie 提取的设备 / 认证上下文
struct Ctx {
    nsm: String,
    cid: String,
    channel: String,
    version: String,
    version_code: String,
    device_id: String,
    ti: String,
    sign: String,
    model: String,
    nnid: String,
    nuid: String,
    csrf: String,
    system_type: String,
    system_version: String,
    token: String,
    session_id: String,
    vip_type: String,
}

impl Ctx {
    fn from_cookie(cookie: &str) -> Self {
        // 不做 URL decode：MUSIC_U 中的 '+' 会被错误地转为空格
        let c = cookie_to_json(cookie);
        let get = |k: &str, d: &str| {
            c.get(k)
                .filter(|v| !v.is_empty())
                .cloned()
                .unwrap_or_else(|| d.to_string())
        };
        Ctx {
            nsm: get("WEVNSM", "1.0.0"),
            cid: c.get("WNMCID").cloned().unwrap_or_else(|| {
                format!(
                    "{}.{}.01.0",
                    hex::encode(rand::random::<[u8; 3]>()),
                    chrono::Utc::now().timestamp_millis()
                )
            }),
            channel: get("channel", "netease"),
            version: get("appver", "3.1.35"),
            version_code: get("versioncode", "205293"),
            device_id: c
                .get("deviceId")
                .or_else(|| c.get("sDeviceId"))
                .cloned()
                .unwrap_or_default(),
            ti: get("NMTID", ""),
            sign: get("clientSign", ""),
            model: c
                .get("mode")
                .or_else(|| c.get("mobilename"))
                .cloned()
                .unwrap_or_default(),
            nnid: get("_ntes_nnid", ","),
            nuid: get("_ntes_nuid", ""),
            csrf: get("__csrf", ""),
            system_type: "pc".to_string(),
            system_version: get(
                "osver",
                "Microsoft-Windows-10-Professional-build-19045-64bit",
            ),
            token: get("MUSIC_U", ""),
            session_id: get("JSESSIONID-WYYY", ""),
            vip_type: get("vipType", ""),
        }
    }

    fn fields(&self) -> Vec<(&'static str, String)> {
        vec![
            ("JSESSIONID-WYYY", self.session_id.clone()),
            ("MUSIC_U", self.token.clone()),
            ("NMTID", self.ti.clone()),
            ("WEVNSM", self.nsm.clone()),
            ("WNMCID", self.cid.clone()),
            ("__csrf", self.csrf.clone()),
            ("_iuqxldmzr_", "33".into()),
            ("_ntes_nnid", self.nnid.clone()),
            ("_ntes_nuid", self.nuid.clone()),
            ("appver", format!("{}.{}", self.version, self.version_code)),
            ("channel", self.channel.clone()),
            ("clientSign", self.sign.clone()),
            ("deviceId", self.device_id.clone()),
            ("mode", self.model.clone()),
            ("ntes_kaola_ad", "1".into()),
            ("os", self.system_type.clone()),
            ("osver", self.system_version.clone()),
        ]
    }

    fn cookie_str(&self) -> String {
        let mut f = self.fields();
        f.insert(7, ("__remember_me", "true".into()));
        f.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("; ")
    }

    fn meta_json(&self) -> String {
        let map: serde_json::Map<String, Value> = self
            .fields()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.into()))
            .collect();
        Value::Object(map).to_string()
    }
}

struct Song {
    id: i64,
    bitrate: i64,
    level: String,
    time: f64,
}

/// 日志记录：time \x01 action \x01 json
fn build_record(time: i64, action: &str, data: &Value) -> String {
    format!("{}\x01{}\x01{}", time, action, data)
}

fn build_plv(ctx: &Ctx, song: &Song, source_id: &str, source_name: &str) -> Value {
    let now = chrono::Utc::now().timestamp_millis();
    json!({
        "mode": "circulation", "download": 0, "alg": "", "status": "front",
        "id": song.id.to_string(), "bitrate": song.bitrate, "type": "song",
        "is_listentogether": 0, "source": source_name, "is_heart": 0,
        "resource_ratio": "", "resource_time": song.time, "musiceffect_id": "",
        "app_mode": 2, "bitrate_level": song.level,
        "_addrefer": format!("[F:63][{now}#933#{v}#{vc}#c9156c3][e][2][23][cell_pc_songlist_song:2|page_pc_songlist_songflow|page_mine_like_music][{id}:song:x:x|:::|{sid}:list::]",
            v = ctx.version, vc = ctx.version_code, id = song.id, sid = source_id),
        "_multirefers": [
            "[F:26][s][18][_ai]",
            "[F:26][s][12][_ai]",
            format!("[F:63][{now}#933#{}#{}#c9156c3][e][2][8][cell_pc_main_tab_entrance:6|page_pc_main_tab][我喜欢的音乐:spm::|:::]", ctx.version, ctx.version_code),
            "[F:26][s][5][_ai]",
            "[F:26][s][0][_ai]"
        ],
        "vipType": ctx.vip_type, "fee": 1, "file": 4, "rightSource": 0,
        "sourceId": source_id, "sourcetype": "track", "libra_abt": "",
        "channel": ctx.channel, "curStartChannel": ""
    })
}

fn build_pld(ctx: &Ctx, song: &Song, source_id: &str, source_name: &str, played: f64) -> Value {
    let now = chrono::Utc::now().timestamp_millis();
    json!({
        "mode": "circulation", "download": 0, "alg": "", "status": "front",
        "id": song.id.to_string(), "time": played, "type": "song",
        "is_listentogether": 0, "source": source_name, "is_heart": 0,
        "realtime": played, "resource_ratio": "", "resource_time": song.time,
        "musiceffect_id": "1001", "app_mode": 1, "lyriceffect": "default",
        "displayMode": "classic", "bitrate": song.bitrate, "bitrate_level": song.level,
        "_addrefer": format!("[F:63][{now}#616#{v}#{vc}#c9156c3][e][2][92][btn_pc_cover_play|cell_pc_songlist_song:6|page_pc_songlist_songflow|page_mine_like_music][:::|{id}:song:x:x|:::|{sid}:list::]",
            v = ctx.version, vc = ctx.version_code, id = song.id, sid = source_id),
        "_multirefers": ["[F:26][s][87][_ai]", "[F:26][s][81][_ai]", "[F:26][s][75][_ai]", "[F:26][s][69][_ai]", "[F:26][s][63][_ai]"],
        "vipType": ctx.vip_type, "fee": 8, "file": 4, "rightSource": 0,
        "sourceId": source_id, "sourcetype": "track", "end": "interrupt",
        "libra_abt": "", "channel": ctx.channel, "curStartChannel": ""
    })
}

struct UploadResult {
    success: bool,
    file_name: String,
    payload_size: usize,
    resp: Value,
}

impl ApiClient {
    async fn ncbl_upload(&self, ctx: &Ctx, body: &str) -> Result<UploadResult> {
        let payload = ncbl_encrypt(ctx.meta_json().as_bytes(), body.as_bytes());
        let boundary = hex::encode(rand::random::<[u8; 16]>());
        let file_name = {
            let mut rng = rand::rng();
            format!(
                "op_{}_0_{}",
                rng.random_range(10000..100000),
                rng.random_range(1..=u32::MAX as u64)
            )
        };
        let mut multipart = format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{file_name}\"\r\nContent-Type: multipart/form-data\r\n\r\n"
        )
        .into_bytes();
        multipart.extend_from_slice(&payload);
        multipart.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());

        let resp: Value = self
            .client
            .post(format!("{}/api/clientlog/encrypt/upload?multiupload=true", CL_DOMAIN3))
            .header("Content-Type", format!("multipart/form-data; boundary={boundary}"))
            .header("Referer", "https://music.163.com/di")
            .header(
                "User-Agent",
                format!("Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Safari/537.36 Chrome/91.0.4472.164 NeteaseMusicDesktop/{}", ctx.version),
            )
            .header("Accept-Language", "zh-CN,zh;q=0.8")
            .header("Cookie", ctx.cookie_str())
            .timeout(std::time::Duration::from_secs(15))
            .body(multipart)
            .send()
            .await?
            .json()
            .await
            .unwrap_or(Value::Null);

        let success = resp.get("code").and_then(|c| c.as_i64()) == Some(200)
            && resp
                .pointer("/data/successfiles")
                .and_then(|v| v.as_array())
                .is_some_and(|a| a.iter().any(|f| f.as_str() == Some(&file_name)));
        Ok(UploadResult {
            success,
            file_name,
            payload_size: payload.len(),
            resp,
        })
    }

    /// 听歌打卡（NCBL 加密版，PLV 与 PLD 分两次上传）
    /// 对应 /scrobble/v1
    ///
    /// 参数：id 歌曲 id, time 播放时长（秒）, total 歌曲总时长, sourceid 来源 id, source 来源名（默认 list）,
    /// bitrate（默认 320）, level（默认 exhigh）。需要登录（MUSIC_U）
    pub async fn scrobble_v1(&self, query: &Query) -> Result<ApiResponse> {
        let bad = |status: i64, msg: &str| {
            Ok(ApiResponse {
                status,
                body: json!({ "code": status, "msg": msg }),
                cookie: vec![],
            })
        };
        let song_id = query.get_i64("id", 0);
        if song_id == 0 {
            return bad(400, "缺少有效的 id (歌曲ID)");
        }
        let play_time = query.get_or("time", "").parse::<f64>().unwrap_or(0.0);
        if play_time <= 0.0 {
            return bad(400, "缺少有效的 time (播放时长)");
        }
        let total_time = query
            .get_or("total", "")
            .parse::<f64>()
            .ok()
            .filter(|t| *t > 0.0)
            .unwrap_or(play_time);

        let ctx = Ctx::from_cookie(query.cookie.as_deref().unwrap_or(""));
        if ctx.token.is_empty() {
            return bad(401, "缺少 MUSIC_U 鉴权令牌");
        }

        let song = Song {
            id: song_id,
            bitrate: query.get_i64("bitrate", 320),
            level: query.get_or("level", "exhigh"),
            time: total_time,
        };
        let source_id = query
            .get("sourceid")
            .or_else(|| query.get("sourceId"))
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| song_id.to_string());
        let source_name = query.get_or("source", "list");

        let ts = chrono::Utc::now().timestamp();
        let played = play_time.min(total_time);
        let plv_body = build_record(
            ts,
            "_plv",
            &build_plv(&ctx, &song, &source_id, &source_name),
        );
        let pld_body = build_record(
            ts,
            "_pld",
            &build_pld(&ctx, &song, &source_id, &source_name, played),
        );

        let plv = self.ncbl_upload(&ctx, &plv_body).await?;
        let resp_code = |r: &Value| r.get("code").cloned().unwrap_or(json!(-1));
        if !plv.success {
            let rate = plv
                .resp
                .pointer("/data/rate")
                .filter(|v| !v.is_null())
                .map(|r| format!(" (rate={})", r))
                .unwrap_or_default();
            return Ok(ApiResponse {
                status: 200,
                body: json!({ "code": resp_code(&plv.resp), "msg": format!("PLV 上报失败{}", rate), "details": plv.resp }),
                cookie: vec![],
            });
        }
        let pld = self.ncbl_upload(&ctx, &pld_body).await?;
        if !pld.success {
            return Ok(ApiResponse {
                status: 200,
                body: json!({
                    "code": resp_code(&pld.resp),
                    "msg": "PLV 成功但 PLD 失败",
                    "details": { "plv": plv.resp, "pld": pld.resp }
                }),
                cookie: vec![],
            });
        }
        Ok(ApiResponse {
            status: 200,
            body: json!({
                "code": 200,
                "data": "scrobble_v1 上报成功",
                "details": {
                    "plv": { "fileName": plv.file_name, "payloadSize": plv.payload_size },
                    "pld": { "fileName": pld.file_name, "payloadSize": pld.payload_size }
                }
            }),
            cookie: vec![],
        })
    }
}
