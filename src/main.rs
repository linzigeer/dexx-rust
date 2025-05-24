mod config;
mod utils;

use config::Config;
use utils::AppResult;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> AppResult<()> {
    // 初始化日志
    fmt::init();
    
    // 打印版本信息
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("dexx-rust version 0.1.0");
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!();
    
    // 创建必要的目录
    tokio::fs::create_dir_all("./logs").await?;
    tokio::fs::create_dir_all("./space").await?;
    
    // 加载配置
    let config = Config::new()?;
    tracing::info!("Configuration loaded successfully");
    
    tracing::info!("Starting HTTP server on {}", config.http_listen);
    
    // 简单的HTTP服务器
    let app = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Dexx Rust Server v0.1.0" }));
    
    let listener = tokio::net::TcpListener::bind(&config.http_listen).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
