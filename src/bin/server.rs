use ncm_api::server::{ServerConfig, start_server};

#[tokio::main]
async fn main() {
    let config = ServerConfig::from_env();
    println!(
        "Starting NCM API Rust Server on {}:{}",
        config.host, config.port
    );
    start_server(config).await;
}
