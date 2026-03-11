//! 构建脚本 - 自动从 src/api/mod.rs 扫描模块声明，生成路由注册代码
//!
//! 类似 Node.js 版自动扫描 module/ 目录注册路由

use std::fs;
use std::io::Write;
use std::path::Path;

/// 不转换下划线为斜杠的特殊路由（与 Node.js server.js 中的 special 对应）
const SPECIAL_ROUTES: &[&str] = &[
    "daily_signin",
    "fm_trash",
    "personal_fm",
    "personal_fm_mode",
];

/// 需要特殊处理的路由，不走通用宏
/// - 上传路由：需要 multipart 处理
/// - 非标准签名：eapi_decrypt (同步方法), inner_version (无 query 参数)
/// - 一个文件多个方法的模块名（文件名 ≠ 方法名）
const EXCLUDED_MODULES: &[&str] = &[
    "avatar_upload",
    "voice_upload",
    "eapi_decrypt",
    "inner_version",
    // 以下模块的文件名和方法名不一致（一个文件包含多个方法）
    "cloud",                 // -> cloud_upload_check, cloud_upload_info, cloud_publish
    "cloud_upload_token",    // -> cloud_upload_token_check, cloud_upload_token_alloc
    "cloud_upload_complete", // -> cloud_upload_complete_info, cloud_upload_complete_pub
];

fn method_to_route(method: &str) -> String {
    if SPECIAL_ROUTES.contains(&method) {
        format!("/{}", method)
    } else {
        format!("/{}", method.replace('_', "/"))
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src/api/mod.rs");

    let mod_rs = fs::read_to_string("src/api/mod.rs").expect("Failed to read src/api/mod.rs");

    // 提取所有 `mod xxx;` 声明
    let mut methods: Vec<String> = Vec::new();
    for line in mod_rs.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("mod ") {
            if let Some(name) = rest.strip_suffix(';') {
                let name = name.trim();
                if !EXCLUDED_MODULES.contains(&name) {
                    methods.push(name.to_string());
                }
            }
        }
    }

    // 生成路由注册代码
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("api_routes_generated.rs");
    let mut f = fs::File::create(&dest_path).expect("Failed to create generated file");

    writeln!(f, "// 自动生成的路由注册代码 - 请勿手动修改").unwrap();
    writeln!(f, "// 由 build.rs 从 src/api/mod.rs 扫描生成").unwrap();
    writeln!(f, "//").unwrap();
    writeln!(f, "// 共 {} 个标准路由", methods.len()).unwrap();
    writeln!(f).unwrap();
    writeln!(f, "api_routes!(router,").unwrap();

    for method in methods.iter() {
        let route = method_to_route(method);
        writeln!(f, "    {} => \"{}\",", method, route).unwrap();
    }

    writeln!(f, ")").unwrap();
}
