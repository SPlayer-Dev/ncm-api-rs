/// 请求模块 - 对应 Node.js 版本的 util/request.js
///
/// 核心功能：构造加密请求、Cookie 管理、UA 伪装
use crate::crypto;
use crate::error::{NcmError, Result};
use crate::util::config::*;
use crate::util::cookie::{cookie_obj_to_string, cookie_to_json};
use crate::util::device::{generate_device_id, generate_wnmcid, random_hex};
use crate::util::ip::generate_random_chinese_ip;

/// randomCNIP 使用的进程级随机中国 IP（与 Node 版 global.cnIp 一致，启动时生成一次）
static CN_IP: LazyLock<String> = LazyLock::new(generate_random_chinese_ip);

use reqwest::header::{HeaderMap, HeaderValue, COOKIE, REFERER, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::LazyLock;

/// 特殊状态码集合（视为 200）
static SPECIAL_STATUS_CODES: LazyLock<std::collections::HashSet<i64>> =
    LazyLock::new(|| [201, 302, 400, 502, 800, 801, 802, 803].into());

/// 全局设备 ID（进程生命周期内固定）
static DEVICE_ID: LazyLock<String> = LazyLock::new(generate_device_id);
/// 全局 WNMCID
static WNMCID: LazyLock<String> = LazyLock::new(generate_wnmcid);
/// Cookie Domain 移除正则（编译一次，全局复用）
static DOMAIN_REGEX: LazyLock<regex_lite::Regex> =
    LazyLock::new(|| regex_lite::Regex::new(r"\s*Domain=[^;]+;?").unwrap());

mod transport;

use transport::{client_builder, send_with_retry};

/// 安全创建 HeaderValue，无效字符会被过滤
fn header_value(s: &str) -> HeaderValue {
    HeaderValue::from_str(s).unwrap_or_else(|_| {
        // 过滤掉非 ASCII 可见字符
        let safe: String = s
            .chars()
            .filter(|c| c.is_ascii() && !c.is_ascii_control())
            .collect();
        HeaderValue::from_str(&safe).unwrap_or_else(|_| HeaderValue::from_static(""))
    })
}

/// NMTID：服务端在不带 NMTID 的 eapi 请求中下发，采集后全局复用
static NMTID: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
static NMTID_RETRIES_LEFT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(3);

/// xeapi 公钥（首次使用时拉取）
static XEAPI_PUBLIC_KEY: tokio::sync::Mutex<Option<crypto::XeapiPublicKey>> =
    tokio::sync::Mutex::const_new(None);
/// xeapi 会话 (x-encr-ssid, x-encr-sskey)
static XEAPI_SESSION: std::sync::Mutex<Option<(String, String)>> = std::sync::Mutex::new(None);

/// neapi 密钥缓存（配置会轮换，过期后后台刷新）
struct NeapiKeyState {
    key: crypto::NeapiKey,
    circle_time: i64,
    fetched_at: i64,
}
static NEAPI_KEY: tokio::sync::Mutex<Option<NeapiKeyState>> = tokio::sync::Mutex::const_new(None);
/// 服务端 x-user-status 响应头（空值同样记录）
static NEAPI_USER_STATUS: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

/// 加密类型
#[derive(Debug, Clone, PartialEq, Default)]
pub enum CryptoType {
    Weapi,
    #[default]
    Eapi,
    Linuxapi,
    Api, // 明文
    Xeapi,
    Neapi,
}

/// 反作弊 token（X-antiCheatToken）
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CheckToken {
    #[default]
    None,
    /// 静态 token（Node 版 v2 依赖 jsdom 运行易盾 SDK，这里以静态 token 兜底）
    Static,
    /// 易盾 v3 实时 token
    V3,
}

impl CryptoType {
    pub fn as_str(&self) -> &str {
        match self {
            CryptoType::Weapi => "weapi",
            CryptoType::Eapi => "eapi",
            CryptoType::Linuxapi => "linuxapi",
            CryptoType::Api => "api",
            CryptoType::Xeapi => "xeapi",
            CryptoType::Neapi => "neapi",
        }
    }
}

impl From<&str> for CryptoType {
    fn from(s: &str) -> Self {
        match s {
            "weapi" => CryptoType::Weapi,
            "linuxapi" => CryptoType::Linuxapi,
            "api" => CryptoType::Api,
            "xeapi" => CryptoType::Xeapi,
            "neapi" => CryptoType::Neapi,
            _ => CryptoType::Eapi,
        }
    }
}

/// 请求选项
#[derive(Debug, Clone, Default)]
pub struct RequestOption {
    pub crypto: CryptoType,
    pub cookie: Option<String>,
    pub ua: Option<String>,
    pub proxy: Option<String>,
    pub real_ip: Option<String>,
    pub random_cn_ip: bool,
    pub e_r: Option<bool>,
    pub domain: Option<String>,
    pub check_token: CheckToken,
    /// 额外请求头（会被内部计算的同名头覆盖）
    pub headers: HashMap<String, String>,
    /// 请求超时（毫秒），None / 0 表示不设置
    pub timeout: Option<u64>,
}

/// API 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: i64,
    pub body: Value,
    #[serde(default)]
    pub cookie: Vec<String>,
}

/// API 客户端
#[derive(Debug, Clone)]
pub struct ApiClient {
    pub(crate) client: reqwest::Client,
    cookie: Option<String>,
    anonymous_token: Option<String>,
    /// 自定义设备 ID，None 则使用全局默认值
    device_id: Option<String>,
}

impl ApiClient {
    /// 创建新的 API 客户端
    pub fn new(cookie: Option<String>) -> Self {
        let client = client_builder()
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            cookie,
            anonymous_token: None,
            device_id: None,
        }
    }

    /// 创建带代理的 API 客户端
    pub fn with_proxy(cookie: Option<String>, proxy_url: &str) -> Result<Self> {
        let proxy = reqwest::Proxy::all(proxy_url)
            .map_err(|e| NcmError::Unknown(format!("Invalid proxy URL: {}", e)))?;
        let client = client_builder()
            .proxy(proxy)
            .build()
            .map_err(NcmError::Http)?;

        Ok(Self {
            client,
            cookie,
            anonymous_token: None,
            device_id: None,
        })
    }

    /// 设置 cookie
    pub fn set_cookie(&mut self, cookie: String) {
        self.cookie = Some(cookie);
    }

    /// 设置匿名 token
    pub fn set_anonymous_token(&mut self, token: String) {
        self.anonymous_token = Some(token);
    }

    /// 设置自定义设备 ID
    ///
    /// 不同客户端实例可使用不同设备 ID，避免共享全局 ID 触发风控
    pub fn set_device_id(&mut self, device_id: String) {
        self.device_id = Some(device_id);
    }

    /// 获取当前设备 ID（自定义优先，否则使用全局默认）
    pub fn device_id(&self) -> &str {
        self.device_id.as_deref().unwrap_or(&DEVICE_ID)
    }

    /// 获取易盾 v3 反作弊 token（每次实时获取，不缓存）
    pub(crate) async fn fetch_check_token_v3(&self) -> Result<String> {
        static RE: LazyLock<regex_lite::Regex> =
            LazyLock::new(|| regex_lite::Regex::new(r#"null\(\[(\d+),\d+,"([^"]+)"\]\)"#).unwrap());
        let body = self
            .client
            .get(format!("{}/v3/b?pn=YD00000558929251", DUN_DOMAIN_V3))
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?
            .text()
            .await?;
        match RE.captures(&body) {
            Some(c) if &c[1] == "200" => Ok(c[2].to_string()),
            _ => Err(NcmError::Unknown(format!(
                "易盾返回异常: {}",
                body.chars().take(100).collect::<String>()
            ))),
        }
    }

    /// 向服务端申请 xeapi 公钥（对应 Node.js module/register_xeapikey.js）
    pub(crate) async fn fetch_xeapi_public_key(
        &self,
        current_key_version: &str,
    ) -> Result<crypto::XeapiPublicKey> {
        let nonce: String = (0..16)
            .map(|_| char::from(b'0' + rand::random::<u8>() % 10))
            .collect();
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let device_id = self.device_id().to_string();
        let form = [
            ("appVersion", "9.5.61"),
            ("currentKeyVersion", current_key_version),
            ("deviceId", &device_id),
            ("nonce", &nonce),
            ("os", "android"),
            ("requestType", "active"),
            ("signature", &crypto::xeapi_sign(&timestamp, &nonce)),
            ("t1", ""),
            ("t2", ""),
            ("timestamp", &timestamp),
            ("uid", ""),
        ];
        let res: Value = self
            .client
            .post(format!("{}/api/bsr/sk/get", API_DOMAIN))
            .header(USER_AGENT, choose_user_agent("api", "android"))
            .header(
                COOKIE,
                format!("deviceId={}", urlencoding::encode(&device_id)),
            )
            .form(&form)
            .send()
            .await?
            .json()
            .await?;

        let data = res
            .get("data")
            .filter(|_| res.get("code").and_then(|c| c.as_i64()) == Some(200))
            .ok_or_else(|| NcmError::Crypto("xeapi public key request failed".into()))?;
        let field = |k: &str| match data.get(k) {
            Some(Value::String(s)) => s.clone(),
            Some(v) if !v.is_null() => v.to_string(),
            _ => String::new(),
        };
        if field("signature").is_empty()
            || crypto::xeapi_sign(&field("timestamp"), &nonce) != field("signature")
        {
            return Err(NcmError::Crypto(
                "xeapi public key response signature mismatch".into(),
            ));
        }
        let pk =
            crypto::xeapi_decrypt_public_key(&field("encryptedData")).map_err(NcmError::Crypto)?;
        if pk.sk.is_empty() {
            return Err(NcmError::Crypto(
                "xeapi public key response missing sk".into(),
            ));
        }
        Ok(pk)
    }

    /// 获取（必要时刷新）缓存的 xeapi 公钥
    pub(crate) async fn xeapi_public_key(&self) -> Result<crypto::XeapiPublicKey> {
        let mut guard = XEAPI_PUBLIC_KEY.lock().await;
        if let Some(pk) = guard.as_ref() {
            return Ok(pk.clone());
        }
        let pk = self.fetch_xeapi_public_key("").await?;
        *guard = Some(pk.clone());
        Ok(pk)
    }

    /// 强制刷新 xeapi 公钥
    pub(crate) async fn refresh_xeapi_public_key(&self) -> Result<crypto::XeapiPublicKey> {
        let mut guard = XEAPI_PUBLIC_KEY.lock().await;
        let current = guard
            .as_ref()
            .map(|p| p.version.clone())
            .unwrap_or_default();
        let pk = self.fetch_xeapi_public_key(&current).await?;
        *guard = Some(pk.clone());
        *XEAPI_SESSION.lock().unwrap() = None;
        Ok(pk)
    }

    /// 拉取 neapi 配置（对应 Node.js module/register_neapikey.js）
    ///
    /// 返回 (新配置, circleTime, 服务端版本)；file 为空表示本地已是最新，新配置为 None
    pub(crate) async fn fetch_neapi_key(
        &self,
        version: u16,
        behavior: &str,
    ) -> Result<(Option<crypto::NeapiKey>, i64, Value)> {
        let res: Value = self
            .client
            .get(format!("{}/api/gorilla/algorithm/record", API_DOMAIN))
            .query(&[
                ("version", version.to_string().as_str()),
                ("behavior", behavior),
            ])
            .header(USER_AGENT, choose_user_agent("api", "android"))
            .send()
            .await?
            .json()
            .await?;
        let data = res
            .get("data")
            .filter(|_| res.get("code").and_then(|c| c.as_i64()) == Some(200))
            .ok_or_else(|| NcmError::Crypto("neapi config request failed".into()))?;
        let circle_time = data.get("circleTime").and_then(|v| v.as_i64()).unwrap_or(0);
        let key = match data
            .get("file")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            Some(file) => {
                use base64::Engine;
                let raw = base64::engine::general_purpose::STANDARD
                    .decode(file)
                    .map_err(|e| NcmError::Crypto(e.to_string()))?;
                Some(crypto::neapi_decode_config(&raw).map_err(NcmError::Crypto)?)
            }
            None => None,
        };
        Ok((
            key,
            circle_time,
            data.get("version").cloned().unwrap_or(Value::Null),
        ))
    }

    /// 刷新 neapi 配置并写入缓存，返回 (是否更新, circleTime, 当前版本)
    pub(crate) async fn refresh_neapi_key(&self) -> Result<(bool, i64, Value)> {
        let mut guard = NEAPI_KEY.lock().await;
        let (version, behavior) = match guard.as_ref() {
            Some(s) => (s.key.version, "update"),
            None => (0, "open"),
        };
        let (key, circle_time, server_version) = self.fetch_neapi_key(version, behavior).await?;
        let now = chrono::Utc::now().timestamp_millis();
        let updated = key.is_some();
        match (key, guard.as_mut()) {
            (Some(key), _) => {
                *guard = Some(NeapiKeyState {
                    key,
                    circle_time,
                    fetched_at: now,
                })
            }
            (None, Some(state)) => {
                state.circle_time = circle_time;
                state.fetched_at = now;
            }
            (None, None) => return Err(NcmError::Crypto("neapi config is missing".into())),
        }
        let version = guard
            .as_ref()
            .map(|s| Value::from(s.key.version))
            .unwrap_or(server_version);
        Ok((updated, circle_time, version))
    }

    /// 获取缓存的 neapi 配置；超过 circleTime 时后台刷新，本次仍用手上的配置
    async fn neapi_key(&self) -> Result<crypto::NeapiKey> {
        {
            let guard = NEAPI_KEY.lock().await;
            if let Some(state) = guard.as_ref() {
                if chrono::Utc::now().timestamp_millis() - state.fetched_at >= state.circle_time {
                    let client = self.clone();
                    tokio::spawn(async move {
                        let _ = client.refresh_neapi_key().await;
                    });
                }
                return Ok(state.key.clone());
            }
        }
        self.refresh_neapi_key().await?;
        NEAPI_KEY
            .lock()
            .await
            .as_ref()
            .map(|s| s.key.clone())
            .ok_or_else(|| NcmError::Crypto("neapi config is missing".into()))
    }

    /// 发起 API 请求 - 核心方法
    pub async fn request(
        &self,
        uri: &str,
        data: Value,
        options: RequestOption,
    ) -> Result<ApiResponse> {
        if options.crypto != CryptoType::Neapi {
            return self.request_once(uri, data, options).await;
        }
        let res = self.request_once(uri, data.clone(), options.clone()).await;
        let status = NEAPI_USER_STATUS.lock().unwrap().clone();
        if status == "777" || status == "999" {
            // 配置已轮换：刷新后重试一次
            self.refresh_neapi_key().await?;
            return self.request_once(uri, data, options).await;
        }
        res
    }

    async fn request_once(
        &self,
        uri: &str,
        data: Value,
        options: RequestOption,
    ) -> Result<ApiResponse> {
        let mut headers = HeaderMap::new();
        for (k, v) in &options.headers {
            if let Ok(name) = reqwest::header::HeaderName::from_bytes(k.as_bytes()) {
                headers.insert(name, header_value(v));
            }
        }

        // IP 伪装
        let ip = options.real_ip.clone().or_else(|| {
            if options.random_cn_ip {
                Some(CN_IP.clone())
            } else {
                None
            }
        });

        if let Some(ref ip) = ip {
            if let (Ok(real_ip), Ok(fwd)) = (HeaderValue::from_str(ip), HeaderValue::from_str(ip)) {
                headers.insert("X-Real-IP", real_ip);
                headers.insert("X-Forwarded-For", fwd);
            }
        }

        // Cookie 处理
        let cookie_str = options
            .cookie
            .as_deref()
            .or(self.cookie.as_deref())
            .unwrap_or("");
        let mut cookie_map = cookie_to_json(cookie_str);

        // 注入必要的 cookie 字段
        let ntes_nuid = random_hex(32);
        let os = get_os_config(cookie_map.get("os").map(|s| s.as_str()).unwrap_or("pc"));
        let now_ts = chrono::Utc::now().timestamp_millis().to_string();

        cookie_map
            .entry("__remember_me".to_string())
            .or_insert_with(|| "true".to_string());
        cookie_map
            .entry("ntes_kaola_ad".to_string())
            .or_insert_with(|| "1".to_string());
        cookie_map
            .entry("_ntes_nnid".to_string())
            .or_insert_with(|| format!("{},{}", ntes_nuid, now_ts));
        cookie_map
            .entry("_ntes_nuid".to_string())
            .or_insert(ntes_nuid);
        cookie_map
            .entry("WNMCID".to_string())
            .or_insert_with(|| WNMCID.clone());
        cookie_map
            .entry("WEVNSM".to_string())
            .or_insert_with(|| "1.0.0".to_string());
        cookie_map
            .entry("osver".to_string())
            .or_insert_with(|| os.osver.to_string());
        cookie_map
            .entry("deviceId".to_string())
            .or_insert_with(|| self.device_id().to_string());
        cookie_map
            .entry("os".to_string())
            .or_insert_with(|| os.os.to_string());
        cookie_map
            .entry("channel".to_string())
            .or_insert_with(|| os.channel.to_string());
        cookie_map
            .entry("appver".to_string())
            .or_insert_with(|| os.appver.to_string());

        // 确定加密方式
        let crypto_type = if options.crypto == CryptoType::Eapi && !ENCRYPT {
            CryptoType::Api
        } else {
            options.crypto.clone()
        };

        // NMTID：服务端只会在不带 NMTID 的 eapi 请求里下发，此时留空用于采集
        let has_user_nmtid = cookie_map.contains_key("NMTID");
        if !has_user_nmtid {
            let global = NMTID.lock().unwrap().clone();
            if !global.is_empty() {
                cookie_map.insert("NMTID".to_string(), global);
            } else if NMTID_RETRIES_LEFT.load(Ordering::Relaxed) <= 0
                || crypto_type != CryptoType::Eapi
            {
                cookie_map.insert("NMTID".to_string(), format!("00O{}", random_hex(19)));
            }
        }
        let probe_nmtid = crypto_type == CryptoType::Eapi
            && !cookie_map.contains_key("NMTID")
            && NMTID_RETRIES_LEFT.load(Ordering::Relaxed) > 0;

        // 反作弊 token
        let anti_cheat_token = match options.check_token {
            CheckToken::None => None,
            CheckToken::Static => Some(CHECK_TOKEN.to_string()),
            CheckToken::V3 => Some(self.fetch_check_token_v3().await.unwrap_or_default()),
        };

        // 未登录时注入匿名 token
        if !cookie_map.contains_key("MUSIC_U") {
            if let Some(ref token) = self.anonymous_token {
                cookie_map
                    .entry("MUSIC_A".to_string())
                    .or_insert_with(|| token.clone());
            }
        }

        headers.insert(COOKIE, header_value(&cookie_obj_to_string(&cookie_map)));

        let mut data = data;
        // e_r：options > data.e_r > 配置默认值，所有加密方式都写入 data（与 Node 版一致）
        let e_r = options
            .e_r
            .or_else(|| match data.get("e_r") {
                Some(Value::Bool(b)) => Some(*b),
                Some(Value::String(s)) => Some(s == "true"),
                _ => None,
            })
            .unwrap_or(ENCRYPT_RESPONSE);
        if data.is_object() {
            data["e_r"] = Value::Bool(e_r);
        }
        let url: String;
        let encrypt_data: HashMap<String, String>;
        // neapi 的请求体是容器本身，不是表单
        let mut raw_body: Option<String> = None;
        let mut neapi_key: Option<crypto::NeapiKey> = None;
        let domain = options.domain.as_deref().unwrap_or("");

        let csrf_token = cookie_map.get("__csrf").cloned().unwrap_or_default();

        match crypto_type {
            CryptoType::Weapi => {
                let ref_domain = if domain.is_empty() { DOMAIN } else { domain };
                headers.insert(REFERER, header_value(ref_domain));
                let ua = options
                    .ua
                    .as_deref()
                    .unwrap_or_else(|| choose_user_agent("weapi", "pc"));
                headers.insert(USER_AGENT, header_value(ua));
                if let Some(ref token) = anti_cheat_token {
                    headers.insert("X-antiCheatToken", header_value(token));
                }

                data["csrf_token"] = Value::String(csrf_token);
                encrypt_data = crypto::weapi(&data);
                url = format!("{}/weapi/{}", ref_domain, &uri[5..]);
            }
            CryptoType::Linuxapi => {
                let ua = options
                    .ua
                    .as_deref()
                    .unwrap_or_else(|| choose_user_agent("linuxapi", "linux"));
                headers.insert(USER_AGENT, header_value(ua));

                let ref_domain = if domain.is_empty() { DOMAIN } else { domain };
                let linux_data = serde_json::json!({
                    "method": "POST",
                    "url": format!("{}{}", ref_domain, uri),
                    "params": data,
                });
                encrypt_data = crypto::linuxapi(&linux_data);
                url = format!("{}/api/linux/forward", ref_domain);
            }
            CryptoType::Xeapi => {
                let pk = self.xeapi_public_key().await?;
                let is_android = cookie_map.get("os").map(|s| s.as_str()) == Some("android");
                let xe_appver = cookie_map
                    .get("appver")
                    .filter(|_| is_android)
                    .cloned()
                    .unwrap_or_else(|| "9.1.65".to_string());
                let xe_osver = cookie_map
                    .get("osver")
                    .filter(|_| is_android)
                    .cloned()
                    .unwrap_or_else(|| "16".to_string());
                let xe_buildver = cookie_map
                    .get("buildver")
                    .cloned()
                    .unwrap_or_else(|| chrono::Utc::now().timestamp().to_string());
                let device_id = cookie_map.get("deviceId").cloned().unwrap_or_default();
                let s_device_id = cookie_map
                    .get("sDeviceId")
                    .cloned()
                    .unwrap_or_else(|| device_id.clone());

                let ua = options
                    .ua
                    .as_deref()
                    .unwrap_or_else(|| choose_user_agent("api", "android"));
                headers.insert(USER_AGENT, header_value(ua));
                headers.insert("X-Client-Enc-State", HeaderValue::from_static("ENCRYPTED"));
                headers.insert("x-aeapi", HeaderValue::from_static("true"));
                headers.insert("x-deviceid", header_value(&device_id));
                headers.insert("x-os", HeaderValue::from_static("android"));
                headers.insert("x-osver", header_value(&xe_osver));
                headers.insert("x-appver", header_value(&xe_appver));
                headers.insert("x-sdeviceid", header_value(&s_device_id));
                headers.insert("x-buildver", header_value(&xe_buildver));
                if let Some(music_u) = cookie_map.get("MUSIC_U") {
                    headers.insert("x-music-u", header_value(music_u));
                }
                if let Some(ref token) = anti_cheat_token {
                    headers.insert("X-antiCheatToken", header_value(token));
                }

                let mut xe_cookie = cookie_map.clone();
                xe_cookie.insert("os".to_string(), "android".to_string());
                xe_cookie.insert("osver".to_string(), xe_osver);
                xe_cookie.insert("appver".to_string(), xe_appver);
                xe_cookie.insert("buildver".to_string(), xe_buildver);
                xe_cookie.insert("sDeviceId".to_string(), s_device_id);
                headers.insert(COOKIE, header_value(&cookie_obj_to_string(&xe_cookie)));

                let session = XEAPI_SESSION.lock().unwrap().clone();
                encrypt_data = crypto::xeapi(
                    &data,
                    &pk,
                    session
                        .as_ref()
                        .map(|(id, key)| (id.as_str(), key.as_str())),
                    "android",
                )
                .map_err(NcmError::Crypto)?;
                let xe_domain = if domain.is_empty() {
                    XEAPI_DOMAIN
                } else {
                    domain
                };
                url = format!("{}/xeapi/{}", xe_domain, &uri[5..]);
            }
            CryptoType::Eapi | CryptoType::Api | CryptoType::Neapi => {
                // 构造 eapi header cookie
                let now_secs = chrono::Utc::now().timestamp().to_string();
                let request_id = format!(
                    "{}_{:04}",
                    chrono::Utc::now().timestamp_millis(),
                    rand::random::<u16>() % 1000
                );

                let mut header_map: HashMap<String, String> = HashMap::new();
                header_map.insert(
                    "osver".to_string(),
                    cookie_map.get("osver").cloned().unwrap_or_default(),
                );
                header_map.insert(
                    "deviceId".to_string(),
                    cookie_map.get("deviceId").cloned().unwrap_or_default(),
                );
                header_map.insert(
                    "os".to_string(),
                    cookie_map.get("os").cloned().unwrap_or_default(),
                );
                header_map.insert(
                    "appver".to_string(),
                    cookie_map.get("appver").cloned().unwrap_or_default(),
                );
                header_map.insert(
                    "versioncode".to_string(),
                    cookie_map
                        .get("versioncode")
                        .cloned()
                        .unwrap_or_else(|| "140".to_string()),
                );
                header_map.insert(
                    "mobilename".to_string(),
                    cookie_map.get("mobilename").cloned().unwrap_or_default(),
                );
                header_map.insert(
                    "buildver".to_string(),
                    cookie_map
                        .get("buildver")
                        .cloned()
                        .unwrap_or_else(|| now_secs[..10].to_string()),
                );
                header_map.insert(
                    "resolution".to_string(),
                    cookie_map
                        .get("resolution")
                        .cloned()
                        .unwrap_or_else(|| "1920x1080".to_string()),
                );
                header_map.insert("__csrf".to_string(), csrf_token.clone());
                header_map.insert(
                    "channel".to_string(),
                    cookie_map.get("channel").cloned().unwrap_or_default(),
                );
                header_map.insert("requestId".to_string(), request_id);

                if let (Some(token), false) = (&anti_cheat_token, crypto_type == CryptoType::Neapi)
                {
                    header_map.insert("X-antiCheatToken".to_string(), token.clone());
                }
                if crypto_type == CryptoType::Eapi {
                    if let Some(nmtid) = cookie_map.get("NMTID") {
                        header_map.insert("NMTID".to_string(), nmtid.clone());
                    }
                }

                if let Some(music_u) = cookie_map.get("MUSIC_U") {
                    header_map.insert("MUSIC_U".to_string(), music_u.clone());
                }
                if let Some(music_a) = cookie_map.get("MUSIC_A") {
                    header_map.insert("MUSIC_A".to_string(), music_a.clone());
                }

                let header_cookie_str = header_map
                    .iter()
                    .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                    .collect::<Vec<_>>()
                    .join("; ");

                headers.insert(COOKIE, header_value(&header_cookie_str));

                let ua = options.ua.as_deref().unwrap_or_else(|| {
                    if cookie_map.get("os").map(|s| s.as_str()) == Some("osx") {
                        choose_user_agent("weapi", "pc")
                    } else {
                        choose_user_agent("api", "iphone")
                    }
                });
                headers.insert(USER_AGENT, header_value(ua));

                let api_domain = if !domain.is_empty() {
                    domain
                } else if crypto_type == CryptoType::Eapi {
                    EAPI_DOMAIN
                } else {
                    API_DOMAIN
                };

                if crypto_type == CryptoType::Neapi {
                    let key = self.neapi_key().await?;
                    let ua = options
                        .ua
                        .as_deref()
                        .unwrap_or_else(|| choose_user_agent("api", "android"));
                    headers.insert(USER_AGENT, header_value(ua));
                    // 服务端按 X-ER 选密钥，配置轮换时由 777/999 分支重拉并重试
                    headers.insert("X-ER", header_value(&key.version.to_string()));
                    let status = NEAPI_USER_STATUS.lock().unwrap().clone();
                    if !status.is_empty() {
                        headers.insert("X-USER-STATUS", header_value(&status));
                    }
                    let pairs: Vec<(String, String)> = data
                        .as_object()
                        .map(|m| {
                            m.iter()
                                .filter(|(k, _)| k.as_str() != "e_r")
                                .map(|(k, v)| (k.clone(), crypto::value_to_form_string(v)))
                                .collect()
                        })
                        .unwrap_or_default();
                    let plain = serde_urlencoded::to_string(&pairs)
                        .map_err(|e| NcmError::Unknown(e.to_string()))?;
                    raw_body =
                        Some(crypto::neapi(plain.as_bytes(), &key).map_err(NcmError::Crypto)?);
                    neapi_key = Some(key);
                    encrypt_data = HashMap::new();
                    let ne_domain = if domain.is_empty() {
                        NEAPI_DOMAIN
                    } else {
                        domain
                    };
                    url = format!("{}/neapi/{}", ne_domain, &uri[5..]);
                } else if crypto_type == CryptoType::Eapi {
                    // 注入 header
                    let header_value = serde_json::to_value(&header_map).unwrap();
                    data["header"] = header_value;

                    encrypt_data = crypto::eapi(uri, &data);
                    url = format!("{}/eapi/{}", api_domain, &uri[5..]);
                } else {
                    // api 明文
                    encrypt_data = if let Value::Object(map) = &data {
                        map.iter()
                            .map(|(k, v)| {
                                (
                                    k.clone(),
                                    match v {
                                        Value::String(s) => s.clone(),
                                        _ => v.to_string(),
                                    },
                                )
                            })
                            .collect()
                    } else {
                        HashMap::new()
                    };
                    url = format!("{}{}", api_domain, uri);
                }
            }
        }

        // 构造 POST body
        let body = match raw_body {
            Some(b) => b,
            None => serde_urlencoded::to_string(&encrypt_data)
                .map_err(|e| NcmError::Unknown(e.to_string()))?,
        };

        // 设置 Content-Type
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        // 构造请求并发送（按需使用代理）
        let proxy_client;
        let client = if let Some(ref proxy_url) = options.proxy {
            let proxy = reqwest::Proxy::all(proxy_url)
                .map_err(|e| NcmError::Unknown(format!("Invalid proxy URL: {}", e)))?;
            proxy_client = client_builder()
                .proxy(proxy)
                .build()
                .map_err(NcmError::Http)?;
            &proxy_client
        } else {
            &self.client
        };
        let mut req = client.post(&url).headers(headers).body(body);
        if let Some(ms) = options.timeout.filter(|&ms| ms > 0) {
            req = req.timeout(std::time::Duration::from_millis(ms));
        }
        let response = send_with_retry(req).await.map_err(|e| {
            if e.is_timeout() {
                NcmError::Timeout(e.to_string())
            } else {
                NcmError::Http(e)
            }
        })?;

        // 处理响应 cookie
        let resp_cookies: Vec<String> = response
            .headers()
            .get_all("set-cookie")
            .iter()
            .filter_map(|v| v.to_str().ok())
            .map(|s| {
                // 移除 Domain 属性
                DOMAIN_REGEX.replace_all(s, "").to_string()
            })
            .collect();

        // 采集服务端下发的 NMTID
        if probe_nmtid && !has_user_nmtid {
            NMTID_RETRIES_LEFT.fetch_sub(1, Ordering::Relaxed);
            if let Some(v) = resp_cookies.iter().find_map(|c| {
                c.split(';')
                    .find_map(|kv| kv.trim().strip_prefix("NMTID="))
                    .map(str::to_string)
            }) {
                *NMTID.lock().unwrap() = v;
            }
        }

        if crypto_type == CryptoType::Neapi {
            *NEAPI_USER_STATUS.lock().unwrap() = response
                .headers()
                .get("x-user-status")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();
        }

        if crypto_type == CryptoType::Xeapi {
            let h = response.headers();
            if let (Some(id), Some(key)) = (
                h.get("x-encr-ssid").and_then(|v| v.to_str().ok()),
                h.get("x-encr-sskey").and_then(|v| v.to_str().ok()),
            ) {
                *XEAPI_SESSION.lock().unwrap() = Some((id.to_string(), key.to_string()));
            }
        }

        // 解析响应体
        let status_code = response.status().as_u16() as i64;

        let body: Value = if let Some(ref key) = neapi_key {
            let text = response.text().await?;
            crypto::neapi_res_decrypt(&text, key)
                .unwrap_or_else(|_| serde_json::from_str(&text).unwrap_or(Value::String(text)))
        } else if crypto_type == CryptoType::Xeapi {
            let bytes = response.bytes().await?;
            crypto::xeapi_res_decrypt(&bytes).unwrap_or_else(|_| {
                let text = String::from_utf8_lossy(&bytes).to_string();
                serde_json::from_str(&text).unwrap_or(Value::String(text))
            })
        } else if matches!(crypto_type, CryptoType::Eapi | CryptoType::Weapi) && e_r {
            let bytes = response.bytes().await?;
            let hex_str = hex::encode_upper(&bytes);
            crypto::eapi_res_decrypt(&hex_str).unwrap_or(Value::Null)
        } else {
            let text = response.text().await?;
            serde_json::from_str(&text).unwrap_or(Value::String(text))
        };

        let mut status = body
            .get("code")
            .and_then(|c| {
                c.as_i64()
                    .or_else(|| c.as_str().and_then(|s| s.parse().ok()))
            })
            .unwrap_or(status_code);

        // 特殊状态码视为 200
        if SPECIAL_STATUS_CODES.contains(&status) {
            status = 200;
        }

        // 状态码范围检查
        if !(100..600).contains(&status) {
            status = 400;
        }

        let answer = ApiResponse {
            status,
            body,
            cookie: resp_cookies,
        };

        if status == 200 {
            Ok(answer)
        } else {
            let msg = ["msg", "message"]
                .iter()
                .find_map(|k| answer.body.get(*k).and_then(|m| m.as_str()))
                .filter(|m| !m.is_empty())
                .unwrap_or("Unknown error")
                .to_string();
            Err(NcmError::Response {
                msg,
                response: Box::new(answer),
            })
        }
    }
}
