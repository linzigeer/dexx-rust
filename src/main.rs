mod config;
mod utils;
mod models;
mod repositories;
mod services;
mod handlers;

use std::sync::Arc;
use config::Config;
use utils::AppResult;
use repositories::RepositoriesImpl;
use services::ServicesImpl;
use handlers::{AppState, create_routes, middleware::*};
use tracing_subscriber::fmt;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> AppResult<()> {
    // 初始化日志
    fmt::init();
    
    // 打印版本信息
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("dexx-rust version 0.1.0 - API Server");
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!();
    
    // 创建必要的目录
    tokio::fs::create_dir_all("./logs").await?;
    tokio::fs::create_dir_all("./space").await?;
    
    // 加载配置
    let config = Arc::new(Config::new()?);
    tracing::info!("Configuration loaded successfully");
    
    // 初始化数据库和Redis连接
    let repositories = RepositoriesImpl::new(config.clone()).await?;
    tracing::info!("Database and Redis connections established");
    
    // 初始化服务层
    let services = ServicesImpl::new(config.clone(), repositories).await?;
    tracing::info!("Services initialized successfully");
    
    // 创建应用状态
    let app_state = AppState {
        config: config.clone(),
        services,
    };
    
    // 创建路由
    let app = create_routes(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive()) // 添加CORS支持
                .layer(axum::middleware::from_fn(logging_middleware)) // 添加日志中间件
                .layer(axum::middleware::from_fn(error_handling_middleware)) // 添加错误处理中间件
        );
    
    tracing::info!("Starting HTTP server on {}", config.http_listen);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind(&config.http_listen).await?;
    tracing::info!("🚀 Server started successfully!");
    tracing::info!("📡 Health check: http://{}/health", config.http_listen);
    tracing::info!("👤 User API: http://{}/user/*", config.http_listen);
    tracing::info!("🔗 Solana API: http://{}/v2/solana/*", config.http_listen);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
