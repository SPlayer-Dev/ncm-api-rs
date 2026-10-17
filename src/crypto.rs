/// 加密模块 - 对应 Node.js 版本的 util/crypto.js
///
/// 实现三种加密方式：
/// - weapi: 双层 AES-128-CBC + RSA
/// - eapi: MD5 签名 + AES-128-ECB
/// - linuxapi: AES-128-ECB
/// - xeapi: AES-ECB + X25519/AES-GCM 会话密钥
use aes::cipher::{block_padding::Pkcs7, BlockModeDecrypt, BlockModeEncrypt, KeyInit, KeyIvInit};
use md5::{Digest, Md5};
use num_bigint::BigUint;
use rand::RngExt;
use serde::Deserialize;
use std::collections::HashMap;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;

const IV: &[u8] = b"0102030405060708";
const PRESET_KEY: &[u8] = b"0CoJUm6Qyw8W8jud";
const LINUXAPI_KEY: &[u8] = b"rFgB&h#%2?^eDg:Q";
const EAPI_KEY: &[u8] = b"e82ckenh8dichen8";
const BASE62: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

// weapi RSA 公钥（1024 位，e = 65537）的模数 n，取自其 SubjectPublicKeyInfo DER：
// MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ37BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvaklV8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44oncaTWz7OBGLbCiK45wIDAQAB
const WEAPI_RSA_N: &str = "e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7";

/// AES-128-CBC 加密，输出 Base64
fn aes_cbc_encrypt_base64(plaintext: &[u8], key: &[u8], iv: &[u8]) -> String {
    let cipher = Aes128CbcEnc::new_from_slices(key, iv).expect("AES-128 key / iv length");
    let ciphertext = cipher.encrypt_padded_vec::<Pkcs7>(plaintext);
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &ciphertext)
}

/// AES-128-ECB 加密，输出大写 Hex
fn aes_ecb_encrypt_hex(plaintext: &[u8], key: &[u8]) -> String {
    let cipher = Aes128EcbEnc::new_from_slice(key).expect("AES-128 key length");
    let ciphertext = cipher.encrypt_padded_vec::<Pkcs7>(plaintext);
    hex::encode_upper(&ciphertext)
}

/// AES-128-ECB 解密（输入大写 Hex）
fn aes_ecb_decrypt_hex(ciphertext_hex: &str, key: &[u8]) -> Result<Vec<u8>, String> {
    let ciphertext = hex::decode(ciphertext_hex).map_err(|e| e.to_string())?;
    let cipher = Aes128EcbDec::new_from_slice(key).expect("AES-128 key length");
    cipher
        .decrypt_padded_vec::<Pkcs7>(&ciphertext)
        .map_err(|e| e.to_string())
}

/// RSA 加密（NONE / raw / textbook RSA，无 padding）
/// 对应 Node.js: forge.publicKey.encrypt(str, 'NONE')
fn rsa_encrypt_no_padding(plaintext: &[u8]) -> String {
    let n = BigUint::parse_bytes(WEAPI_RSA_N.as_bytes(), 16).unwrap();

    // Textbook RSA: c = m^e mod n
    let m = BigUint::from_bytes_be(plaintext);
    let c = m.modpow(&BigUint::from(65537u32), &n);

    // 输出固定长度 hex（与模数等长，256 hex chars for 1024-bit key）
    let n_bytes = (n.bits() / 8) as usize;
    let c_bytes = c.to_bytes_be();

    // 左侧填充 0
    let mut padded = vec![0u8; n_bytes - c_bytes.len()];
    padded.extend_from_slice(&c_bytes);
    hex::encode(&padded)
}

/// weapi 加密
/// 双层 AES-128-CBC + RSA 加密随机密钥
pub fn weapi(object: &serde_json::Value) -> HashMap<String, String> {
    let text = serde_json::to_string(object).unwrap();
    let mut rng = rand::rng();

    // 生成 16 位随机密钥
    let secret_key: String = (0..16)
        .map(|_| BASE62[rng.random_range(0..62)] as char)
        .collect();

    // 第一层 AES-CBC：preset_key + iv
    let first_encrypt = aes_cbc_encrypt_base64(text.as_bytes(), PRESET_KEY, IV);
    // 第二层 AES-CBC：secret_key + iv
    let params = aes_cbc_encrypt_base64(first_encrypt.as_bytes(), secret_key.as_bytes(), IV);

    // RSA 加密反转后的 secret_key
    let reversed_key: String = secret_key.chars().rev().collect();
    let enc_sec_key = rsa_encrypt_no_padding(reversed_key.as_bytes());

    let mut result = HashMap::new();
    result.insert("params".to_string(), params);
    result.insert("encSecKey".to_string(), enc_sec_key);
    result
}

/// linuxapi 加密
/// 单层 AES-128-ECB
pub fn linuxapi(object: &serde_json::Value) -> HashMap<String, String> {
    let text = serde_json::to_string(object).unwrap();
    let mut result = HashMap::new();
    result.insert(
        "eparams".to_string(),
        aes_ecb_encrypt_hex(text.as_bytes(), LINUXAPI_KEY),
    );
    result
}

/// eapi 加密
/// MD5 签名 + AES-128-ECB
pub fn eapi(url: &str, object: &serde_json::Value) -> HashMap<String, String> {
    let text = serde_json::to_string(object).unwrap();
    let message = format!("nobody{}use{}md5forencrypt", url, text);
    let digest = hex::encode(Md5::digest(message.as_bytes()));
    let data = format!("{}-36cd479b6b5-{}-36cd479b6b5-{}", url, text, digest);

    let mut result = HashMap::new();
    result.insert(
        "params".to_string(),
        aes_ecb_encrypt_hex(data.as_bytes(), EAPI_KEY),
    );
    result
}

/// eapi 响应解密
pub fn eapi_res_decrypt(encrypted_hex: &str) -> Option<serde_json::Value> {
    let decrypted = aes_ecb_decrypt_hex(encrypted_hex, EAPI_KEY).ok()?;
    let text = String::from_utf8(decrypted).ok()?;
    serde_json::from_str(&text).ok()
}

/// linuxapi 请求解密（调试用）
pub fn linuxapi_req_decrypt(encrypted_hex: &str) -> Option<serde_json::Value> {
    let decrypted = aes_ecb_decrypt_hex(encrypted_hex, LINUXAPI_KEY).ok()?;
    serde_json::from_slice(&decrypted).ok()
}

/// eapi 请求解密（调试用）
pub fn eapi_req_decrypt(encrypted_hex: &str) -> Option<(String, serde_json::Value)> {
    let decrypted = aes_ecb_decrypt_hex(encrypted_hex, EAPI_KEY).ok()?;
    let text = String::from_utf8(decrypted).ok()?;

    // 按 "-36cd479b6b5-" 分隔符拆分
    let parts: Vec<&str> = text.splitn(3, "-36cd479b6b5-").collect();
    if parts.len() >= 2 {
        let url = parts[0].to_string();
        let data: serde_json::Value = serde_json::from_str(parts[1]).ok()?;
        Some((url, data))
    } else {
        None
    }
}

// ============================================================
//  xeapi
// ============================================================

const XEAPI_STATIC_KEY: [u8; 32] = [
    0xab, 0x1d, 0x5a, 0x43, 0x0f, 0x6b, 0xb0, 0x4a, 0x3f, 0x01, 0xe8, 0x1d, 0xdd, 0x72, 0xbd, 0x91,
    0x6d, 0x5c, 0xe5, 0x91, 0x24, 0x8a, 0xc1, 0x28, 0x71, 0x48, 0x06, 0xd7, 0xf8, 0xfb, 0x1b, 0x84,
];
const XEAPI_SIGN_KEY: &[u8] =
    b"mUHCwVNWJbunMqAHf5MImuirT6plvs6VSFW62MGHstFQxhBGdEoIhLItH3djc4+FB/OKty3+lL2rGeoFBpVe5g==";

/// xeapi 服务端公钥状态（来自 /api/bsr/sk/get）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XeapiPublicKey {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(default)]
    pub sk: String,
    #[serde(deserialize_with = "de_string_or_number")]
    pub version: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

fn de_string_or_number<'de, D: serde::Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    let v = serde_json::Value::deserialize(d)?;
    Ok(match v {
        serde_json::Value::String(s) => s,
        other => other.to_string(),
    })
}

/// 按密钥长度选择 AES-128/192/256-ECB 加密（PKCS7）
fn aes_ecb_encrypt_any(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, String> {
    Ok(match key.len() {
        16 => ecb::Encryptor::<aes::Aes128>::new_from_slice(key)
            .expect("key length checked")
            .encrypt_padded_vec::<Pkcs7>(plaintext),
        24 => ecb::Encryptor::<aes::Aes192>::new_from_slice(key)
            .expect("key length checked")
            .encrypt_padded_vec::<Pkcs7>(plaintext),
        32 => ecb::Encryptor::<aes::Aes256>::new_from_slice(key)
            .expect("key length checked")
            .encrypt_padded_vec::<Pkcs7>(plaintext),
        n => return Err(format!("invalid aes key length: {}", n)),
    })
}

fn aes_ecb_decrypt_any(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    let r = match key.len() {
        16 => ecb::Decryptor::<aes::Aes128>::new_from_slice(key)
            .expect("key length checked")
            .decrypt_padded_vec::<Pkcs7>(ciphertext),
        24 => ecb::Decryptor::<aes::Aes192>::new_from_slice(key)
            .expect("key length checked")
            .decrypt_padded_vec::<Pkcs7>(ciphertext),
        32 => ecb::Decryptor::<aes::Aes256>::new_from_slice(key)
            .expect("key length checked")
            .decrypt_padded_vec::<Pkcs7>(ciphertext),
        n => return Err(format!("invalid aes key length: {}", n)),
    };
    r.map_err(|e| e.to_string())
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{KeyInit as _, Mac};
    let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(key).expect("hmac key");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// xeapi 签名：HMAC-SHA256(signKey, timestamp + nonce)，Base64
pub fn xeapi_sign(timestamp: &str, nonce: &str) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(hmac_sha256(
        XEAPI_SIGN_KEY,
        format!("{}{}", timestamp, nonce).as_bytes(),
    ))
}

/// 解密 /api/bsr/sk/get 返回的 encryptedData
pub fn xeapi_decrypt_public_key(encrypted_data: &str) -> Result<XeapiPublicKey, String> {
    use base64::Engine;
    let raw = base64::engine::general_purpose::STANDARD
        .decode(encrypted_data)
        .map_err(|e| e.to_string())?;
    let plain = aes_ecb_decrypt_any(&XEAPI_STATIC_KEY, &raw)?;
    serde_json::from_slice(&plain).map_err(|e| e.to_string())
}

fn xeapi_mid_transform(ciphertext: &[u8]) -> Vec<u8> {
    let random: [u8; 16] = rand::random();
    let xored: Vec<u8> = ciphertext
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ random[i & 0x0f])
        .collect();
    let rot = if xored.is_empty() {
        0
    } else {
        (random[0] & 0x0f) as usize % xored.len()
    };
    let mut out = random.to_vec();
    out.extend_from_slice(&xored[rot..]);
    out.extend_from_slice(&xored[..rot]);
    out
}

/// X25519 临时密钥协商，返回 (临时公钥, 共享密钥)
fn x25519_ephemeral_agree(peer: [u8; 32]) -> ([u8; 32], [u8; 32]) {
    let mut os_rng = rand::rand_core::UnwrapErr(rand::rngs::SysRng);
    let secret = x25519_dalek::EphemeralSecret::random_from_rng(&mut os_rng);
    let ephemeral = x25519_dalek::PublicKey::from(&secret);
    let shared = secret.diffie_hellman(&x25519_dalek::PublicKey::from(peer));
    (ephemeral.to_bytes(), shared.to_bytes())
}

/// AES-128-GCM 加密，输出 密文 + 16 字节 tag
fn aes128_gcm_seal(key: &[u8], iv: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>, String> {
    use aes_gcm::aead::{Aead, KeyInit as _};
    let cipher = aes_gcm::Aes128Gcm::new_from_slice(key).map_err(|e| e.to_string())?;
    cipher
        .encrypt(iv.into(), plaintext)
        .map_err(|e| e.to_string())
}

fn xeapi_encrypt_s(dynamic_key: &[u8], pk: &XeapiPublicKey, os: &str) -> Result<Vec<u8>, String> {
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD;

    let peer: [u8; 32] = b64
        .decode(&pk.public_key)
        .map_err(|e| e.to_string())?
        .try_into()
        .map_err(|_| "invalid x25519 public key".to_string())?;
    let (ephemeral, shared) = x25519_ephemeral_agree(peer);

    // HKDF-SHA256(salt = 0^32)，取前 16 字节
    let prk = hmac_sha256(&[0u8; 32], &shared);
    let mut info = ephemeral.to_vec();
    info.push(1);
    let aes_key = &hmac_sha256(&prk, &info)[..16];

    let iv: [u8; 12] = rand::random();
    let plaintext = format!("{}|{}|{}", b64.encode(dynamic_key), os, pk.sk);
    let sealed = aes128_gcm_seal(aes_key, &iv, plaintext.as_bytes())?;

    let mut out = ephemeral.to_vec();
    out.extend_from_slice(&iv);
    out.extend_from_slice(&sealed);
    Ok(out)
}

/// 构造 xeapi 明文：{"content": <form body>, "queryString": "e_r=true"}
fn build_xeapi_plaintext(data: &serde_json::Value) -> String {
    let pairs: Vec<(String, String)> = data
        .as_object()
        .map(|m| {
            m.iter()
                .filter(|(k, _)| k.as_str() != "e_r")
                .map(|(k, v)| (k.clone(), value_to_form_string(v)))
                .collect()
        })
        .unwrap_or_default();
    let body = serde_urlencoded::to_string(&pairs).unwrap_or_default();
    let mut fields = serde_json::Map::new();
    if !body.is_empty() {
        fields.insert("content".into(), body.into());
    }
    fields.insert("queryString".into(), "e_r=true".into());
    serde_json::Value::Object(fields).to_string()
}

/// 对齐 JS URLSearchParams 的值转换
pub(crate) fn value_to_form_string(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => "null".into(),
        serde_json::Value::Object(_) => "[object Object]".into(),
        serde_json::Value::Array(a) => a
            .iter()
            .map(value_to_form_string)
            .collect::<Vec<_>>()
            .join(","),
        other => other.to_string(),
    }
}

/// xeapi 加密
///
/// - `session`: 服务端下发的 (x-encr-ssid, x-encr-sskey)，有则复用
pub fn xeapi(
    data: &serde_json::Value,
    pk: &XeapiPublicKey,
    session: Option<(&str, &str)>,
    os: &str,
) -> Result<HashMap<String, String>, String> {
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD;

    let dynamic_key: Vec<u8> = match session {
        Some((_, key)) => key.as_bytes().to_vec(),
        None => rand::random::<[u8; 16]>().to_vec(),
    };
    let plaintext = build_xeapi_plaintext(data);

    let inner = aes_ecb_encrypt_any(&XEAPI_STATIC_KEY, plaintext.as_bytes())?;
    let c = aes_ecb_encrypt_any(&dynamic_key, &xeapi_mid_transform(&inner))?;
    let s = xeapi_encrypt_s(&dynamic_key, pk, os)?;
    let r = aes_ecb_encrypt_any(
        &XEAPI_STATIC_KEY,
        format!("{}|{}", pk.version, session.map(|(id, _)| id).unwrap_or("")).as_bytes(),
    )?;

    let mut result = HashMap::new();
    result.insert(
        "C".to_string(),
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(c),
    );
    result.insert("S".to_string(), b64.encode(s));
    result.insert("R".to_string(), b64.encode(r));
    Ok(result)
}

/// xeapi 响应解密：AES-128-ECB(eapiKey) + 可选 gzip
pub fn xeapi_res_decrypt(body: &[u8]) -> Result<serde_json::Value, String> {
    let decrypted = aes_ecb_decrypt_any(EAPI_KEY, body)?;
    let plain = if decrypted.starts_with(&[0x1f, 0x8b]) {
        use std::io::Read;
        let mut out = Vec::new();
        flate2::read::GzDecoder::new(&decrypted[..])
            .read_to_end(&mut out)
            .map_err(|e| e.to_string())?;
        out
    } else {
        decrypted
    };
    serde_json::from_slice(&plain).map_err(|e| e.to_string())
}

// ============================================================
//  zstd / ChaCha20 工具
// ============================================================

pub(crate) fn zstd_compress(data: &[u8]) -> Vec<u8> {
    ruzstd::encoding::compress_to_vec(data, ruzstd::encoding::CompressionLevel::Fastest)
}

/// 服务端下发的 zstd 帧可能没有结束块（最后一个块未置 last 标记），
/// 这里把最后一个完整块补上 last 标记并去掉校验和标记，使标准解码器可以解出
fn zstd_terminate_frame(data: &[u8]) -> Option<Vec<u8>> {
    let fhd = *data.get(4)?;
    let single_segment = fhd & 0x20 != 0;
    let dict_len = [0, 1, 2, 4][(fhd & 3) as usize];
    let fcs_len = match fhd >> 6 {
        0 => single_segment as usize,
        1 => 2,
        2 => 4,
        _ => 8,
    };
    let mut pos = 5 + (!single_segment) as usize + dict_len + fcs_len;
    let mut last_header = None;
    while pos + 3 <= data.len() {
        let h = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], 0]);
        let size = if (h >> 1) & 3 == 1 {
            1
        } else {
            (h >> 3) as usize
        };
        if pos + 3 + size > data.len() {
            break;
        }
        if h & 1 == 1 {
            return None; // 已正常结束
        }
        last_header = Some(pos);
        pos += 3 + size;
    }
    let p = last_header?;
    let mut out = data[..pos].to_vec();
    out[4] &= !0x04; // 去掉 content checksum 标记
    out[p] |= 1;
    Some(out)
}

pub(crate) fn zstd_decompress(data: &[u8]) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let fixed = zstd_terminate_frame(data);
    let mut src = fixed.as_deref().unwrap_or(data);
    let mut dec = ruzstd::decoding::StreamingDecoder::new(&mut src).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    dec.read_to_end(&mut out).map_err(|e| e.to_string())?;
    Ok(out)
}

/// IETF ChaCha20（32 位块计数器 + 12 字节 nonce）
fn chacha20_xor(key: &[u8; 32], counter: u32, nonce: &[u8], data: &[u8]) -> Vec<u8> {
    use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
    let nonce = nonce.try_into().expect("ChaCha20 nonce must be 12 bytes");
    let mut c = chacha20::ChaCha20::new(key.into(), nonce);
    c.seek(counter as u64 * 64);
    let mut out = data.to_vec();
    c.apply_keystream(&mut out);
    out
}

// ============================================================
//  NCBL（客户端日志加密上报，scrobble_v1 使用）
// ============================================================

/// 256 位 RSA 模数（已被分解，网易 NCBL 专用 key wrap）
const NCBL_RSA_N: &str = "fd90bd466ff9bc8a3fec2fbcf263b90d5c564879fa5d7aab89b31c1d5cb4139d";
const NCBL_HEADER_FIXED_LEN: usize = 70;
const NCBL_META_BLOCK_TYPE: u16 = 0x4343;
const NCBL_MAX_FRAME: usize = 0x8000;

/// NCBL 密钥包装：keyB = keyA^65537 mod N，左侧补 0 到 32 字节
fn ncbl_wrap_key(key_a: &[u8; 32]) -> [u8; 32] {
    let n = BigUint::parse_bytes(NCBL_RSA_N.as_bytes(), 16).unwrap();
    let wrapped = BigUint::from_bytes_be(key_a)
        .modpow(&BigUint::from(65537u32), &n)
        .to_bytes_be();
    let mut key_b = [0u8; 32];
    key_b[32 - wrapped.len()..].copy_from_slice(&wrapped);
    key_b
}

/// NCBL 加密：header(70) + meta 块（keyB 加密）+ 帧序列（keyA 加密的 zstd 压缩 body）
pub fn ncbl_encrypt(meta: &[u8], body: &[u8]) -> Vec<u8> {
    let mut key_a: [u8; 32] = rand::random();
    if key_a[0] >= 0xa3 {
        key_a[0] = 0xa2;
    }
    let key_b = ncbl_wrap_key(&key_a);

    let mut uuid: [u8; 16] = rand::random();
    uuid[6] = (uuid[6] & 0x0f) | 0x40;
    uuid[8] = (uuid[8] & 0x3f) | 0x80;
    let nonce = &uuid[..12];
    let counter = u32::from_le_bytes(uuid[12..16].try_into().unwrap()) >> 2;
    let base_seq = rand::random::<u16>() as u32;

    let meta_cipher = chacha20_xor(&key_b, counter, nonce, meta);
    let mut meta_block = NCBL_META_BLOCK_TYPE.to_le_bytes().to_vec();
    meta_block.extend_from_slice(&(meta_cipher.len() as u16).to_le_bytes());
    meta_block.extend_from_slice(&meta_cipher);

    let compressed = zstd_compress(body);
    let mut trailing = Vec::new();
    let mut seq = base_seq;
    let chunks: Vec<&[u8]> = if compressed.is_empty() {
        vec![&[]]
    } else {
        compressed.chunks(NCBL_MAX_FRAME).collect()
    };
    for chunk in chunks {
        let cipher = chacha20_xor(&key_a, counter, nonce, chunk);
        trailing.extend_from_slice(&(cipher.len() as u16).to_le_bytes());
        trailing.extend_from_slice(&seq.to_le_bytes());
        trailing.extend_from_slice(&cipher);
        seq = seq.wrapping_add(1);
    }

    let mut out = Vec::with_capacity(NCBL_HEADER_FIXED_LEN + meta_block.len() + trailing.len());
    out.extend_from_slice(b"NCBL");
    out.extend_from_slice(&3u32.to_le_bytes());
    out.extend_from_slice(&((NCBL_HEADER_FIXED_LEN + meta_block.len()) as u16).to_le_bytes());
    out.extend_from_slice(&uuid);
    out.extend_from_slice(&key_b);
    out.extend_from_slice(&base_seq.to_le_bytes());
    out.extend_from_slice(&seq.wrapping_sub(1).to_le_bytes());
    out.extend_from_slice(&(trailing.len() as u32).to_le_bytes());
    out.extend_from_slice(&meta_block);
    out.extend_from_slice(&trailing);
    out
}

// ============================================================
//  neapi
// ============================================================

/// neapi 密钥配置（来自 /api/gorilla/algorithm/record）
#[derive(Debug, Clone)]
pub struct NeapiKey {
    pub version: u16,
    pub key_blob: Vec<u8>,
    pub encrypt_key: [u8; 32],
    pub decrypt_key: [u8; 32],
}

const NEAPI_MAGIC: &[u8] = b"CSJM";

/// ChaCha20-Poly1305 加密，输出 密文 + 16 字节 tag
fn chacha20poly1305_seal(
    key: &[u8; 32],
    nonce: &[u8; 12],
    aad: &[u8],
    msg: &[u8],
) -> Result<Vec<u8>, String> {
    use chacha20poly1305::aead::{Aead, KeyInit as _, Payload};
    chacha20poly1305::ChaCha20Poly1305::new(key.into())
        .encrypt(nonce.into(), Payload { msg, aad })
        .map_err(|e| e.to_string())
}

/// ChaCha20-Poly1305 解密并校验 tag
fn chacha20poly1305_open(
    key: &[u8; 32],
    nonce: &[u8; 12],
    aad: &[u8],
    sealed: &[u8],
) -> Result<Vec<u8>, String> {
    use chacha20poly1305::aead::{Aead, KeyInit as _, Payload};
    chacha20poly1305::ChaCha20Poly1305::new(key.into())
        .decrypt(nonce.into(), Payload { msg: sealed, aad })
        .map_err(|e| e.to_string())
}

/// neapi 加密：zstd 压缩 + ChaCha20-Poly1305，输出 Base64 容器
pub fn neapi(data: &[u8], key: &NeapiKey) -> Result<String, String> {
    use base64::Engine;
    let nonce: [u8; 12] = rand::random();
    let ad: [u8; 16] = rand::random();
    let sealed = chacha20poly1305_seal(&key.encrypt_key, &nonce, &ad, &zstd_compress(data))?;

    let mut out = NEAPI_MAGIC.to_vec();
    out.extend_from_slice(&key.version.to_be_bytes());
    out.extend_from_slice(&(nonce.len() as u16).to_le_bytes());
    out.extend_from_slice(&nonce);
    out.push(ad.len() as u8);
    out.extend_from_slice(&ad);
    out.extend_from_slice(&(sealed.len() as u32).to_le_bytes());
    out.extend_from_slice(&sealed);
    Ok(base64::engine::general_purpose::STANDARD.encode(out))
}

/// neapi 响应解密
pub fn neapi_res_decrypt(body: &str, key: &NeapiKey) -> Result<serde_json::Value, String> {
    use base64::Engine;
    let raw = base64::engine::general_purpose::STANDARD
        .decode(body.trim())
        .map_err(|e| e.to_string())?;
    let get =
        |r: std::ops::Range<usize>| raw.get(r).ok_or_else(|| "neapi body too short".to_string());
    let nonce_len = u16::from_le_bytes(get(6..8)?.try_into().unwrap()) as usize;
    let nonce = get(8..8 + nonce_len)?;
    let ad_len = *get(8 + nonce_len..9 + nonce_len)?.first().unwrap() as usize;
    let ad = get(9 + nonce_len..9 + nonce_len + ad_len)?;
    let offset = 9 + nonce_len + ad_len;
    let sealed_len = u32::from_le_bytes(get(offset..offset + 4)?.try_into().unwrap()) as usize;
    let sealed = get(offset + 4..offset + 4 + sealed_len)?;
    let nonce: &[u8; 12] = nonce
        .try_into()
        .map_err(|_| "invalid neapi nonce".to_string())?;

    let compressed = chacha20poly1305_open(&key.decrypt_key, nonce, ad, sealed)?;
    let text = String::from_utf8_lossy(&zstd_decompress(&compressed)?).to_string();
    // 部分错误响应会把同一个 JSON 重复拼接两次
    serde_json::from_str(&text).or_else(|e| match text.find("}{") {
        Some(i) => serde_json::from_str(&text[..=i]).map_err(|e| e.to_string()),
        None => Err(e.to_string()),
    })
}

pub use neapi_config::decode_config as neapi_decode_config;

/// neapi 配置解析：RSA 解出 RC4 密钥 → zstd 解压出 ECFG 容器 → 从混淆的 Lua 字节码中提取密钥
mod neapi_config {
    use super::NeapiKey;
    use num_bigint::BigUint;

    /// 512 位 RSA 公钥模数（PKCS#1 DER 中的 n）
    const CONFIG_N: &str = "e4e79c1cb27a9fab7d2c740e99e923ec51c009169ec3a380c60404e01b060f0160cdc631d9fb7fe53b228a5ff11bee42b965d28c6c2036193f4ccdc7ed853d11";
    const CODE_XOR: [u32; 8] = [
        0x94b4f825, 0x977c6795, 0x327de32b, 0x16da5ba1, 0xcf55a2eb, 0x080bb445, 0x021af055,
        0xfc56b8bd,
    ];
    const STR_XOR: [u8; 16] = [
        0xae, 0xfd, 0xa2, 0xd1, 0x83, 0x2b, 0x51, 0x8a, 0x77, 0x41, 0xc3, 0x25, 0xfb, 0x26, 0x87,
        0xf0,
    ];
    const CHUNK_SIGNATURE: &[u8] = &[0x7f, 0x45, 0x4c, 0x46];
    const DIRECTION_ENCRYPT: i64 = 251;
    const DIRECTION_DECRYPT: i64 = 503;
    const OP_LOADK: u32 = 1;
    const OP_GETUPVAL: u32 = 5;
    const OP_EQ: u32 = 31;

    #[derive(Clone)]
    enum Const {
        Int(i64),
        Str(String),
        Other,
    }

    struct Proto {
        code: Vec<u32>,
        consts: Vec<Const>,
        upvals: Vec<(u8, u8)>,
        protos: Vec<Proto>,
    }

    struct Reader<'a> {
        buf: &'a [u8],
        pos: usize,
    }

    type R<T> = Result<T, String>;

    impl Reader<'_> {
        fn take(&mut self, n: usize) -> R<&[u8]> {
            let s = self
                .buf
                .get(self.pos..self.pos + n)
                .ok_or("neapi config truncated")?;
            self.pos += n;
            Ok(s)
        }
        fn u8(&mut self) -> R<u8> {
            Ok(self.take(1)?[0])
        }
        fn u32(&mut self) -> R<u32> {
            Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
        }
        fn i64(&mut self) -> R<i64> {
            Ok(i64::from_le_bytes(self.take(8)?.try_into().unwrap()))
        }
        fn string(&mut self) -> R<Option<String>> {
            let mut size = self.u8()? as usize;
            if size == 0 {
                return Ok(None);
            }
            if size == 0xff {
                size = self.u32()? as usize;
            }
            // 长度比实际字节数多 1（标准 luac 的写法）
            let body: Vec<u8> = self
                .take(size - 1)?
                .iter()
                .enumerate()
                .map(|(i, b)| b ^ STR_XOR[i & 0x0f])
                .collect();
            Ok(Some(String::from_utf8_lossy(&body).into_owned()))
        }
    }

    fn rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
        let mut s: Vec<u8> = (0..=255).collect();
        let mut j = 0u8;
        for i in 0..256 {
            j = j.wrapping_add(s[i]).wrapping_add(key[i % key.len()]);
            s.swap(i, j as usize);
        }
        let (mut i, mut j) = (0u8, 0u8);
        data.iter()
            .map(|b| {
                i = i.wrapping_add(1);
                j = j.wrapping_add(s[i as usize]);
                s.swap(i as usize, j as usize);
                b ^ s[s[i as usize].wrapping_add(s[j as usize]) as usize]
            })
            .collect()
    }

    /// RSA 公钥"解密"（PKCS#1 v1.5 type 1 去填充）
    fn rsa_public_decrypt(block: &[u8]) -> R<Vec<u8>> {
        let n = BigUint::parse_bytes(CONFIG_N.as_bytes(), 16).unwrap();
        let m = BigUint::from_bytes_be(block)
            .modpow(&BigUint::from(65537u32), &n)
            .to_bytes_be();
        // to_bytes_be 去掉了前导 0x00，剩下 01 FF.. 00 data
        if m.first() != Some(&1) {
            return Err("neapi config rsa padding mismatch".into());
        }
        let sep = m
            .iter()
            .skip(1)
            .position(|&b| b == 0)
            .ok_or("neapi config rsa padding")?;
        Ok(m[sep + 2..].to_vec())
    }

    fn load_proto(r: &mut Reader) -> R<Proto> {
        r.string()?; // source
        r.u32()?;
        r.u32()?;
        r.take(3)?; // numparams / isvararg / maxstack

        let n = r.u32()? as usize;
        let mut code = Vec::with_capacity(n);
        for i in 0..n {
            code.push(r.u32()? ^ CODE_XOR[i & 7]);
        }

        // 常量标签：0 nil / 1 boolean / 3 float / 19 integer / 4,20 string
        let n = r.u32()? as usize;
        let mut consts = Vec::with_capacity(n);
        for _ in 0..n {
            consts.push(match r.u8()? {
                0 => Const::Other,
                1 => {
                    r.u8()?;
                    Const::Other
                }
                3 => {
                    r.take(8)?;
                    Const::Other
                }
                19 => Const::Int(r.i64()?),
                _ => r.string()?.map(Const::Str).unwrap_or(Const::Other),
            });
        }

        let n = r.u32()? as usize;
        let mut upvals = Vec::with_capacity(n);
        for _ in 0..n {
            upvals.push((r.u8()?, r.u8()?));
        }

        let n = r.u32()? as usize;
        let mut protos = Vec::with_capacity(n);
        for _ in 0..n {
            protos.push(load_proto(r)?);
        }

        // 调试段跳过
        let n = r.u32()? as usize;
        r.take(4 * n)?;
        for _ in 0..r.u32()? {
            r.string()?;
            r.u32()?;
            r.u32()?;
        }
        for _ in 0..r.u32()? {
            r.string()?;
        }
        Ok(Proto {
            code,
            consts,
            upvals,
            protos,
        })
    }

    /// create_ctx 里「方向 -> 密钥偏移」的分支：
    ///   if direction == <a> then offset = <upvalue> elseif direction == <b> ...
    fn key_offsets(script: &Proto) -> R<(usize, usize)> {
        let create_ctx = script
            .protos
            .first()
            .ok_or("neapi config missing create_ctx")?;
        // 主函数用一连串 LOADK 装载所有标量，据此把 upvalue 追回常量值
        let mut regs = std::collections::HashMap::new();
        for &ins in &script.code {
            if ins & 0x3f == OP_LOADK {
                regs.insert(
                    (ins >> 6) & 0xff,
                    script.consts.get(((ins >> 14) & 0x3ffff) as usize),
                );
            }
        }
        let (mut enc, mut dec, mut pending) = (None, None, None);
        for &ins in &create_ctx.code {
            match ins & 0x3f {
                OP_EQ => {
                    let c = (ins >> 14) & 0x1ff;
                    if c & 0x100 != 0 {
                        if let Some(Const::Int(v)) = create_ctx.consts.get((c & 0xff) as usize) {
                            pending = Some(*v);
                        }
                    }
                }
                OP_GETUPVAL if pending.is_some() => {
                    let up = create_ctx.upvals.get(((ins >> 23) & 0x1ff) as usize);
                    if let Some(&(1, idx)) = up {
                        if let Some(Some(Const::Int(off))) = regs.get(&(idx as u32)) {
                            match pending.take() {
                                Some(DIRECTION_ENCRYPT) => enc = Some(*off as usize),
                                Some(DIRECTION_DECRYPT) => dec = Some(*off as usize),
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        enc.zip(dec)
            .ok_or_else(|| "neapi config script has an unexpected direction table".into())
    }

    pub fn decode_config(raw: &[u8]) -> R<NeapiKey> {
        use base64::Engine;
        let rc4_key = rsa_public_decrypt(raw.get(..64).ok_or("neapi config too short")?)?;
        let size = u32::from_le_bytes(
            raw.get(64..68)
                .ok_or("neapi config too short")?
                .try_into()
                .unwrap(),
        ) as usize;
        let payload = raw.get(68..68 + size).ok_or("neapi config truncated")?;
        let ecfg = super::zstd_decompress(&rc4(&rc4_key, payload))?;

        // ECFG 头 4 字节 + 大端 u16 版本，之后是 Lua 区块
        let version =
            u16::from_be_bytes(ecfg.get(4..6).ok_or("ecfg too short")?.try_into().unwrap());
        let chunk = &ecfg[6..];
        if !chunk.starts_with(CHUNK_SIGNATURE) {
            return Err("neapi config is not a chunk".into());
        }
        let mut r = Reader {
            buf: chunk,
            pos: 33,
        };
        r.u8()?; // 主闭包 upvalue 数量
        let script = load_proto(&mut r)?;

        let (enc_off, dec_off) = key_offsets(&script)?;
        let blob = script
            .consts
            .iter()
            .filter_map(|c| match c {
                Const::Str(s)
                    if s.len() >= 64
                        && s.trim_end_matches('=')
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/')
                        && s.len() - s.trim_end_matches('=').len() <= 2 =>
                {
                    Some(s)
                }
                _ => None,
            })
            .max_by_key(|s| s.len())
            .ok_or("neapi config missing key blob")?;
        let key_blob = base64::engine::general_purpose::STANDARD
            .decode(blob)
            .map_err(|e| e.to_string())?;
        let slice = |off: usize| -> R<[u8; 32]> {
            key_blob
                .get(off..off + 32)
                .ok_or_else(|| "neapi key offset out of range".to_string())
                .map(|s| s.try_into().unwrap())
        };
        Ok(NeapiKey {
            version,
            encrypt_key: slice(enc_off)?,
            decrypt_key: slice(dec_off)?,
            key_blob,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zstd_roundtrip() {
        let data = b"hello neapi hello neapi hello neapi";
        assert_eq!(zstd_decompress(&zstd_compress(data)).unwrap(), data);
    }

    #[test]
    fn test_neapi_roundtrip() {
        let key = NeapiKey {
            version: 0x0102,
            key_blob: vec![],
            encrypt_key: [7u8; 32],
            decrypt_key: [7u8; 32],
        };
        let enc = neapi(br#"{"code":200}"#, &key).unwrap();
        assert_eq!(
            neapi_res_decrypt(&enc, &key).unwrap(),
            serde_json::json!({"code": 200})
        );
    }

    #[test]
    fn test_chacha20_rfc8439() {
        // RFC 8439 2.4.2
        let key: [u8; 32] = core::array::from_fn(|i| i as u8);
        let nonce = [0, 0, 0, 0, 0, 0, 0, 0x4a, 0, 0, 0, 0];
        let out = chacha20_xor(&key, 1, &nonce, b"Ladies and Gentlemen of the class of '99");
        assert_eq!(hex::encode(&out[..8]), "6e2e359a2568f980");
    }

    #[test]
    fn test_ncbl_header() {
        let out = ncbl_encrypt(b"{}", b"body");
        assert_eq!(&out[..4], b"NCBL");
        let header_len = u16::from_le_bytes([out[8], out[9]]) as usize;
        let trailing = u32::from_le_bytes(out[66..70].try_into().unwrap()) as usize;
        assert_eq!(out.len(), header_len + trailing);
    }

    #[test]
    fn test_xeapi_mid_transform_len() {
        let ct = vec![7u8; 48];
        assert_eq!(xeapi_mid_transform(&ct).len(), 64);
    }

    #[test]
    fn test_xeapi_plaintext() {
        let data = serde_json::json!({"id": 1, "e_r": true, "level": "standard"});
        assert_eq!(
            build_xeapi_plaintext(&data),
            r#"{"content":"id=1&level=standard","queryString":"e_r=true"}"#
        );
        assert_eq!(
            build_xeapi_plaintext(&serde_json::json!({})),
            r#"{"queryString":"e_r=true"}"#
        );
    }

    #[test]
    fn test_aes_ecb_roundtrip() {
        let plaintext = b"hello world test";
        let encrypted = aes_ecb_encrypt_hex(plaintext, EAPI_KEY);
        let decrypted = aes_ecb_decrypt_hex(&encrypted, EAPI_KEY).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_weapi_produces_params_and_encseckey() {
        let obj = serde_json::json!({"id": 123});
        let result = weapi(&obj);
        assert!(result.contains_key("params"));
        assert!(result.contains_key("encSecKey"));
        assert!(!result["params"].is_empty());
        // RSA 输出 256 hex chars (1024-bit key)
        assert_eq!(result["encSecKey"].len(), 256);
    }

    #[test]
    fn test_linuxapi_produces_eparams() {
        let obj = serde_json::json!({"method": "POST", "url": "https://music.163.com/api/test"});
        let result = linuxapi(&obj);
        assert!(result.contains_key("eparams"));
        assert!(!result["eparams"].is_empty());
    }

    /// 已知答案测试：固定输入的输出须与依赖升级前逐字节一致
    mod kat {
        use super::super::*;

        const TEXT: &[u8] = br#"{"id":123,"level":"standard"}"#;

        #[test]
        fn aes_cbc_base64() {
            assert_eq!(
                aes_cbc_encrypt_base64(TEXT, PRESET_KEY, IV),
                "tV6ayG8gmrUelBXptDODGFgPPqb8YJW/EaYOB25Ha1E="
            );
        }

        #[test]
        fn aes_ecb_hex() {
            let hex = aes_ecb_encrypt_hex(TEXT, EAPI_KEY);
            assert_eq!(
                hex,
                "865ECC48530DAE3607DB7663E587EC08A494B03B8C312A035D6114CCAFE352FC"
            );
            assert_eq!(aes_ecb_decrypt_hex(&hex, EAPI_KEY).unwrap(), TEXT);
        }

        #[test]
        fn aes_ecb_any_key_sizes() {
            for (len, want) in [
                (
                    16u8,
                    "c412898834a49d3a8fc40292947ce79d7dcddeefef6ec2a51d6b246bc9738b02",
                ),
                (
                    24,
                    "66704f9cdebb1fccc62a6e9353e003a585ea507a7b7454f5a92f35c10ff268b5",
                ),
                (
                    32,
                    "1601682e6bf55438fe0a27d487d17f9ff1ce90005e649b07d75c1551556df3c7",
                ),
            ] {
                let key: Vec<u8> = (0..len).collect();
                let ct = aes_ecb_encrypt_any(&key, TEXT).unwrap();
                assert_eq!(hex::encode(&ct), want, "key len {len}");
                assert_eq!(aes_ecb_decrypt_any(&key, &ct).unwrap(), TEXT);
            }
        }

        #[test]
        fn rsa_no_padding() {
            assert_eq!(rsa_encrypt_no_padding(b"ponmlkjihgfedcba"), "d15a1683c992095d0c234c19966605c5c5964911268bbeda8cb8d08d834913e59d53b32358903a121b5fca784c1f5ae44951fd02524df58ecc98e52cc7cf8689b42c2e93ddf05b0592512d87f5960467e2f086c018849d76014d323500e30f13ef4cafbb0cf5a66731a3f1776c75ca35d0062dac70a3e33245afabcf47938487");
        }

        #[test]
        fn eapi_params() {
            let params = eapi("/api/song/detail", &serde_json::json!({"id": 123}));
            assert_eq!(params["params"], "7D398AA5036D61F11B22021C618C242421D51F26B6A0246E121BFC7B69A3481F62C92B909AF39DBF41E3032D9B8D0AD4255A14BDCC5BC706FCD0882A2B24200CED6B7441C4CAA692CC4F3A374162751C8F60B959A222F7C46E6094FC8DF43285");
        }

        #[test]
        fn linuxapi_eparams() {
            let params = linuxapi(&serde_json::json!({"method": "POST", "url": "/api/x"}));
            assert_eq!(params["eparams"], "A0D9583F4C5FF68DE851D2893A49DE98256E53720F315A26599EC273C87702D0E254137CD4DCFEF7E51A9955D586B94F");
        }

        #[test]
        fn xeapi_sign_hmac() {
            assert_eq!(
                xeapi_sign("1700000000000", "0123456789012345"),
                "Z9T+aFZBr0nWVV5WMzIyYWvtSQUx7ysC9WcxILLfs2c="
            );
        }

        #[test]
        fn aes128_gcm() {
            let key: [u8; 16] = core::array::from_fn(|i| i as u8);
            let iv: [u8; 12] = core::array::from_fn(|i| 0xa0 + i as u8);
            let sealed = aes128_gcm_seal(&key, &iv, TEXT).unwrap();
            assert_eq!(hex::encode(sealed), "d1a451df5cb30238b954976c2364d50c71d4017bf77653cb446215b93f0b7ddd0a3c685d8733c3bf77c2fb3d21");
        }

        #[test]
        fn chacha20poly1305_rfc8439() {
            // RFC 8439 2.8.2
            let key: [u8; 32] = core::array::from_fn(|i| 0x80 + i as u8);
            let nonce = [7, 0, 0, 0, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47];
            let aad = [
                0x50, 0x51, 0x52, 0x53, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7,
            ];
            let msg: &[u8] = b"Ladies and Gentlemen of the class of '99: \
If I could offer you only one tip for the future, sunscreen would be it.";
            let sealed = chacha20poly1305_seal(&key, &nonce, &aad, msg).unwrap();
            assert_eq!(
                hex::encode(&sealed[..16]),
                "d31a8d34648e60db7b86afbc53ef7ec2"
            );
            assert_eq!(
                hex::encode(&sealed[msg.len()..]),
                "1ae10b594f09e26a7e902ecbd0600691"
            );
            assert_eq!(
                chacha20poly1305_open(&key, &nonce, &aad, &sealed).unwrap(),
                msg
            );
        }

        #[test]
        fn ncbl_key_wrap() {
            let key_a: [u8; 32] = core::array::from_fn(|i| 0x10 + i as u8);
            assert_eq!(
                hex::encode(ncbl_wrap_key(&key_a)),
                "b9d5c4b5f40e117c168c77ecde65841890ee4f96d0cf320d6e2970b0610d6def"
            );
        }

        #[test]
        fn x25519_agree_uses_one_secret() {
            // 对端取基点 u=9 时，共享密钥 = X25519(sk, 9) = 自己的临时公钥
            let mut base = [0u8; 32];
            base[0] = 9;
            let (public, shared) = x25519_ephemeral_agree(base);
            assert_eq!(public, shared);
        }
    }

    #[test]
    fn test_eapi_encrypt_decrypt() {
        let url = "/api/song/detail";
        let obj = serde_json::json!({"id": 123});
        let encrypted = eapi(url, &obj);
        let params = &encrypted["params"];

        // 验证能解密回来
        let (dec_url, dec_data) = eapi_req_decrypt(params).unwrap();
        assert_eq!(dec_url, url);
        assert_eq!(dec_data, obj);
    }
}
