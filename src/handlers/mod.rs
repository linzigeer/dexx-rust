use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use crate::config::Config;
use crate::services::ServicesImpl;
use crate::blockchain::BlockchainServices;

pub mod user;
pub mod solana;
pub mod middleware;
pub mod response;

pub use user::*;
pub use solana::*;
pub use middleware::*;

/// API状态结构
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub services: Arc<ServicesImpl>,
    pub blockchain_services: Option<Arc<BlockchainServices>>,
}

/// 创建API路由
pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // 健康检查
        .route("/health", get(health_check))
        .route("/", get(root_handler))
        
        // 用户相关路由
        .nest("/user", user_routes())
        
        // Solana相关路由 (v2 API)
        .nest("/v2/solana", solana_routes())
        
        // 应用状态
        .with_state(state)
}

/// 根路径处理器
async fn root_handler() -> Json<Value> {
    Json(json!({
        "message": "Dexx Rust API Server",
        "version": "0.1.0",
        "status": "running",
        "checkpoint": "6 - 区块链集成"
    }))
}

/// 健康检查处理器
async fn health_check(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let mut services_status = json!({
        "database": "connected", // TODO: 实际检查数据库连接
        "redis": "connected",    // TODO: 实际检查Redis连接
        "api": "running"
    });

    // 检查区块链服务状态
    if let Some(blockchain_services) = &state.blockchain_services {
        // 检查Solana RPC连接
        match blockchain_services.solana_client.health_check().await {
            Ok(_) => {
                services_status["solana_rpc"] = json!("connected");
            }
            Err(_) => {
                services_status["solana_rpc"] = json!("disconnected");
            }
        }

        // 检查价格服务缓存状态
        let (total_cached, fresh_cached) = blockchain_services.price_service.get_cache_stats().await;
        services_status["price_cache"] = json!({
            "total": total_cached,
            "fresh": fresh_cached
        });

        // 检查交易监听状态
        let is_listening = blockchain_services.transaction_listener.is_listening().await;
        services_status["transaction_listener"] = json!(if is_listening { "running" } else { "stopped" });
    } else {
        services_status["blockchain"] = json!("disabled");
    }
    
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "0.1.0",
        "checkpoint": "6 - 区块链集成",
        "services": services_status
    })))
}

/// 用户路由
fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/walletLogin", post(user_wallet_login))
        .route("/emailLogin", post(user_email_login))
        .route("/reg", post(user_register))
        .route("/userinfo", post(user_info).layer(axum::middleware::from_fn_with_state(
            (), jwt_middleware
        )))
        .route("/updPwd", post(user_update_password).layer(axum::middleware::from_fn_with_state(
            (), jwt_middleware
        )))
        .route("/editUsername", post(user_edit_username).layer(axum::middleware::from_fn_with_state(
            (), jwt_middleware
        )))
        .route("/findPwd", post(user_find_password))
}

/// Solana路由
fn solana_routes() -> Router<AppState> {
    Router::new()
        .route("/tokenInfo", post(solana_token_info))
        .route("/tokenPrice", post(solana_token_price))
        .route("/search", post(solana_search))
        .route("/rank", post(solana_rank))
        .route("/tokenHolder", post(solana_token_holder))
        .route("/tradelatest", post(solana_trade_latest))
        .route("/walletPosition", post(solana_wallet_position))
        .route("/tokenPosition", post(solana_token_position))
        .route("/multiTokenInfo", post(solana_multi_token_info))
        .route("/transactionVolume", post(solana_transaction_volume))
        .route("/dailyTransactionVolume", post(solana_daily_transaction_volume))
        // Webhook路由
        .route("/webhook_v1", post(solana_webhook_v1))
        // 新增区块链相关路由
        .route("/walletVerify", post(solana_wallet_verify))
        .route("/generateChallenge", post(solana_generate_challenge))
        .route("/verifyChallenge", post(solana_verify_challenge))
        .route("/blockchainStatus", get(solana_blockchain_status))
        .route("/priceUpdate", post(solana_price_update))
}
