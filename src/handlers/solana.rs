use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::handlers::{response::*, AppState};
use crate::utils::AppResult;
use crate::utils::AppError;
use crate::models::solana::*;
use crate::models::trade::Trade;

/// 代币信息请求
#[derive(Debug, Deserialize)]
pub struct TokenInfoRequest {
    pub mint: String,
}

/// 代币价格请求
#[derive(Debug, Deserialize)]
pub struct TokenPriceRequest {
    pub mint: String,
}

/// 搜索请求
#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub keyword: String,
    pub limit: Option<u32>,
}

/// 排行榜请求
#[derive(Debug, Deserialize)]
pub struct RankRequest {
    pub sort_by: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// 代币持有者请求
#[derive(Debug, Deserialize)]
pub struct TokenHolderRequest {
    pub mint: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// 最新交易请求
#[derive(Debug, Deserialize)]
pub struct TradeLatestRequest {
    pub mint: Option<String>,
    pub limit: Option<u32>,
}

/// 钱包持仓请求
#[derive(Debug, Deserialize)]
pub struct WalletPositionRequest {
    pub wallet: String,
}

/// 代币持仓请求
#[derive(Debug, Deserialize)]
pub struct TokenPositionRequest {
    pub mint: String,
    pub wallet: String,
}

/// 多代币信息请求
#[derive(Debug, Deserialize)]
pub struct MultiTokenInfoRequest {
    pub mints: Vec<String>,
}

/// 交易量请求
#[derive(Debug, Deserialize)]
pub struct TransactionVolumeRequest {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

/// Webhook请求
#[derive(Debug, Deserialize)]
pub struct WebhookRequest {
    pub transaction: serde_json::Value,
}

/// 代币信息处理器
pub async fn solana_token_info(
    State(state): State<AppState>,
    Json(req): Json<TokenInfoRequest>,
) -> AppResult<Json<ApiResponse<Option<SolToken>>>> {
    match state.services.solana_service().get_token_by_mint(&req.mint).await {
        Ok(token) => Ok(Json(success(token))),
        Err(err) => {
            tracing::error!("Failed to get token info: {:?}", err);
            Err(AppError::internal("Service error"))
        }
    }
}

/// 代币价格处理器
pub async fn solana_token_price(
    State(state): State<AppState>,
    Json(req): Json<TokenPriceRequest>,
) -> AppResult<Json<ApiResponse<Option<f64>>>> {
    match state.services.solana_service().get_token_price(&req.mint).await {
        Ok(price) => Ok(Json(success(price))),
        Err(err) => {
            tracing::error!("Failed to get token price: {:?}", err);
            Err(AppError::internal("Service error"))
        }
    }
}

/// 搜索处理器
pub async fn solana_search(
    State(state): State<AppState>,
    Json(req): Json<SearchRequest>,
) -> AppResult<Json<ApiResponse<Vec<SolToken>>>> {
    // TODO: 实现搜索逻辑
    Ok(Json(success(vec![])))
}

/// 排行榜处理器
pub async fn solana_rank(
    State(state): State<AppState>,
    Json(req): Json<RankRequest>,
) -> AppResult<Json<ApiResponse<Vec<SolToken>>>> {
    // TODO: 实现排行榜逻辑
    Ok(Json(success(vec![])))
}

/// 代币持有者处理器
pub async fn solana_token_holder(
    State(state): State<AppState>,
    Json(req): Json<TokenHolderRequest>,
) -> AppResult<Json<ApiResponse<Vec<TokenHolder>>>> {
    // TODO: 实现代币持有者逻辑
    Ok(Json(success(vec![])))
}

/// 最新交易处理器
pub async fn solana_trade_latest(
    State(state): State<AppState>,
    Json(req): Json<TradeLatestRequest>,
) -> AppResult<Json<ApiResponse<Vec<Trade>>>> {
    // TODO: 实现最新交易逻辑
    Ok(Json(success(vec![])))
}

/// 钱包持仓处理器
pub async fn solana_wallet_position(
    State(state): State<AppState>,
    Json(req): Json<WalletPositionRequest>,
) -> AppResult<Json<ApiResponse<Vec<Position>>>> {
    // TODO: 实现钱包持仓逻辑
    Ok(Json(success(vec![])))
}

/// 代币持仓处理器
pub async fn solana_token_position(
    State(state): State<AppState>,
    Json(req): Json<TokenPositionRequest>,
) -> AppResult<Json<ApiResponse<Option<Position>>>> {
    // TODO: 实现代币持仓逻辑
    Ok(Json(success(None)))
}

/// 多代币信息处理器
pub async fn solana_multi_token_info(
    State(state): State<AppState>,
    Json(req): Json<MultiTokenInfoRequest>,
) -> AppResult<Json<ApiResponse<Vec<SolToken>>>> {
    // TODO: 实现多代币信息逻辑
    Ok(Json(success(vec![])))
}

/// 交易量统计处理器
pub async fn solana_transaction_volume(
    State(state): State<AppState>,
    Json(req): Json<TransactionVolumeRequest>,
) -> AppResult<Json<ApiResponse<f64>>> {
    // TODO: 实现交易量统计逻辑
    Ok(Json(success(0.0)))
}

/// 日交易量处理器
pub async fn solana_daily_transaction_volume(
    State(state): State<AppState>,
    Json(req): Json<DailyTransactionVolumeRequest>,
) -> AppResult<Json<ApiResponse<HashMap<String, f64>>>> {
    // TODO: 实现日交易量逻辑
    Ok(Json(success(HashMap::new())))
}

/// Webhook处理器
pub async fn solana_webhook_v1(
    State(state): State<AppState>,
    Json(req): Json<WebhookRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    // TODO: 实现Webhook处理逻辑
    tracing::info!("Received webhook: {:?}", req);
    Ok(Json(success_empty()))
}

#[cfg(test)]
mod tests {
    
    

    #[tokio::test]
    async fn test_token_info() {
        // TODO: 实现代币信息测试
    }

    #[tokio::test]
    async fn test_token_price() {
        // TODO: 实现代币价格测试
    }
}

/// 钱包验证处理器
pub async fn solana_wallet_verify(
    State(state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    tracing::info!("钱包验证请求: {:?}", req);
    
    // 检查区块链服务是否可用
    let blockchain_services = state.blockchain_services
        .as_ref()
        .ok_or_else(|| AppError::internal("区块链服务未启用"))?;
    
    // 提取请求参数
    let wallet_address = req["walletAddress"].as_str()
        .ok_or_else(|| AppError::bad_request("缺少钱包地址"))?;
    let message = req["message"].as_str()
        .ok_or_else(|| AppError::bad_request("缺少签名消息"))?;
    let signature = req["signature"].as_str()
        .ok_or_else(|| AppError::bad_request("缺少签名"))?;
    
    // 创建验证请求
    let verification_request = crate::blockchain::SignatureVerificationRequest {
        wallet_address: wallet_address.to_string(),
        message: message.to_string(),
        signature: signature.to_string(),
    };
    
    // 执行签名验证
    let result = blockchain_services.wallet_verifier.verify_signature(&verification_request)?;
    
    let response_data = serde_json::json!({
        "isValid": result.is_valid,
        "walletAddress": result.wallet_address,
        "error": result.error
    });
    
    Ok(Json(success(response_data)))
}

/// 价格更新处理器
pub async fn solana_price_update(
    State(state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    tracing::info!("价格更新请求: {:?}", req);
    
    // 检查区块链服务是否可用
    let blockchain_services = state.blockchain_services
        .as_ref()
        .ok_or_else(|| AppError::internal("区块链服务未启用"))?;
    
    // 提取代币地址列表
    let mints = req["mints"].as_array()
        .ok_or_else(|| AppError::bad_request("缺少代币地址列表"))?
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    
    if mints.is_empty() {
        return Err(AppError::bad_request("代币地址列表不能为空"));
    }
    
    // 获取价格数据
    let prices = blockchain_services.price_service.get_multiple_token_prices(&mints).await?;
    
    let response_data = serde_json::json!({
        "prices": prices,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "count": prices.len()
    });
    
    Ok(Json(success(response_data)))
}

/// 生成登录挑战处理器
pub async fn solana_generate_challenge(
    State(state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    tracing::info!("生成登录挑战请求: {:?}", req);
    
    // 检查区块链服务是否可用
    let blockchain_services = state.blockchain_services
        .as_ref()
        .ok_or_else(|| AppError::internal("区块链服务未启用"))?;
    
    // 提取钱包地址
    let wallet_address = req["walletAddress"].as_str()
        .ok_or_else(|| AppError::bad_request("缺少钱包地址"))?;
    
    // 生成挑战消息
    let challenge = blockchain_services.wallet_verifier.generate_login_challenge(wallet_address)?;
    
    let response_data = serde_json::json!({
        "challenge": challenge,
        "walletAddress": wallet_address,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    Ok(Json(success(response_data)))
}

/// 验证登录挑战处理器
pub async fn solana_verify_challenge(
    State(state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    tracing::info!("验证登录挑战请求");
    
    // 检查区块链服务是否可用
    let blockchain_services = state.blockchain_services
        .as_ref()
        .ok_or_else(|| AppError::internal("区块链服务未启用"))?;
    
    // 提取请求参数
    let wallet_address = req["walletAddress"].as_str()
        .ok_or_else(|| AppError::bad_request("缺少钱包地址"))?;
    let challenge = req["challenge"].as_str()
        .ok_or_else(|| AppError::bad_request("缺少挑战消息"))?;
    let signature = req["signature"].as_str()
        .ok_or_else(|| AppError::bad_request("缺少签名"))?;
    
    // 验证登录挑战
    let result = blockchain_services.wallet_verifier.verify_login_challenge(
        wallet_address,
        challenge,
        signature,
    )?;
    
    let response_data = serde_json::json!({
        "isValid": result.is_valid,
        "walletAddress": result.wallet_address,
        "error": result.error
    });
    
    Ok(Json(success(response_data)))
}

/// 获取区块链状态处理器
pub async fn solana_blockchain_status(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    tracing::info!("获取区块链状态请求");
    
    // 检查区块链服务是否可用
    let blockchain_services = state.blockchain_services
        .as_ref()
        .ok_or_else(|| AppError::internal("区块链服务未启用"))?;
    
    // 获取各种状态信息
    let mut status = serde_json::json!({
        "enabled": true,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    // 检查Solana RPC连接
    match blockchain_services.solana_client.health_check().await {
        Ok(_) => {
            status["solana_rpc"] = serde_json::json!({
                "status": "connected",
                "url": state.config.solana.rpc_url
            });
            
            // 获取当前区块高度
            if let Ok(slot) = blockchain_services.solana_client.get_slot().await {
                status["current_slot"] = serde_json::json!(slot);
            }
        }
        Err(e) => {
            status["solana_rpc"] = serde_json::json!({
                "status": "disconnected",
                "error": e.to_string()
            });
        }
    }
    
    // 获取价格服务缓存状态
    let (total_cached, fresh_cached) = blockchain_services.price_service.get_cache_stats().await;
    status["price_service"] = serde_json::json!({
        "cache_total": total_cached,
        "cache_fresh": fresh_cached
    });
    
    // 获取交易监听状态
    let is_listening = blockchain_services.transaction_listener.is_listening().await;
    status["transaction_listener"] = serde_json::json!({
        "status": if is_listening { "running" } else { "stopped" }
    });
    
    Ok(Json(success(status)))
}
