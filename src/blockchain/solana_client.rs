//! Solana RPC客户端服务 - HTTP实现
//! 
//! 使用HTTP JSON-RPC调用与Solana区块链交互，避免依赖冲突

use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::config::Config;
use crate::utils::{AppResult, AppError};
use tracing::{info, warn, error, debug};

/// Solana RPC请求结构
#[derive(Debug, Serialize)]
struct RpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Value,
}

/// Solana RPC响应结构
#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    jsonrpc: String,
    id: u64,
    result: Option<T>,
    error: Option<RpcError>,
}

/// RPC错误结构
#[derive(Debug, Deserialize)]
struct RpcError {
    code: i32,
    message: String,
    data: Option<Value>,
}

/// 账户信息结构
#[derive(Debug, Deserialize)]
pub struct AccountInfo {
    pub data: Vec<String>, // Base64编码的数据
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
}

/// 代币账户信息
#[derive(Debug, Clone)]
pub struct TokenAccount {
    pub mint: String,
    pub owner: String,
    pub amount: u64,
    pub decimals: u8,
    pub delegate: Option<String>,
    pub state: u8,
    pub is_native: Option<u64>,
    pub delegated_amount: u64,
    pub close_authority: Option<String>,
}

/// 代币铸造信息
#[derive(Debug, Clone)]
pub struct Mint {
    pub mint_authority: Option<String>,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: Option<String>,
}

/// 代币信息结构
#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub mint: String,
    pub supply: u64,
    pub decimals: u8,
    pub mint_authority: Option<String>,
    pub freeze_authority: Option<String>,
}

/// Solana客户端服务
pub struct SolanaClientService {
    http_client: Client,
    rpc_url: String,
    config: Arc<Config>,
    request_id: std::sync::atomic::AtomicU64,
}

impl SolanaClientService {
    /// 创建新的Solana客户端
    pub async fn new(config: Arc<Config>) -> AppResult<Self> {
        let rpc_url = config.solana.rpc_url.clone();
        let timeout = Duration::from_secs(config.solana.timeout_seconds.unwrap_or(30));
        
        let http_client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| AppError::BlockchainError(format!("创建HTTP客户端失败: {}", e)))?;

        let client = Self {
            http_client,
            rpc_url: rpc_url.clone(),
            config,
            request_id: std::sync::atomic::AtomicU64::new(1),
        };

        // 测试连接
        match client.health_check().await {
            Ok(_) => info!("Solana RPC连接成功: {}", rpc_url),
            Err(e) => {
                error!("Solana RPC连接失败: {}", e);
                return Err(AppError::BlockchainError(format!("RPC连接失败: {}", e)));
            }
        }

        Ok(client)
    }

    /// 发送RPC请求
    async fn send_rpc_request<T>(&self, method: &str, params: Value) -> AppResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request_id = self.request_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            method: method.to_string(),
            params,
        };

        debug!("发送RPC请求: {} - {}", method, request_id);

        let response = self.http_client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::BlockchainError(format!("RPC请求失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::BlockchainError(format!("RPC响应错误: {}", response.status())));
        }

        let rpc_response: RpcResponse<T> = response
            .json()
            .await
            .map_err(|e| AppError::BlockchainError(format!("解析RPC响应失败: {}", e)))?;

        if let Some(error) = rpc_response.error {
            return Err(AppError::BlockchainError(format!("RPC错误: {} - {}", error.code, error.message)));
        }

        rpc_response.result.ok_or_else(|| {
            AppError::BlockchainError("RPC响应中缺少result字段".to_string())
        })
    }

    /// 获取账户信息
    pub async fn get_account_info(&self, pubkey: &str) -> AppResult<Option<AccountInfo>> {
        let params = json!([
            pubkey,
            {
                "encoding": "base64",
                "commitment": "confirmed"
            }
        ]);

        match self.send_rpc_request::<Value>("getAccountInfo", params).await {
            Ok(result) => {
                if result["value"].is_null() {
                    Ok(None)
                } else {
                    let account_info: AccountInfo = serde_json::from_value(result["value"].clone())
                        .map_err(|e| AppError::BlockchainError(format!("解析账户信息失败: {}", e)))?;
                    Ok(Some(account_info))
                }
            }
            Err(e) => {
                warn!("获取账户信息失败: {}", e);
                Ok(None)
            }
        }
    }

    /// 解析代币账户数据
    fn parse_token_account_data(&self, data: &[u8]) -> AppResult<TokenAccount> {
        if data.len() < 165 { // SPL Token账户最小长度
            return Err(AppError::BlockchainError("代币账户数据长度不足".to_string()));
        }

        // 简化的代币账户解析（实际应该使用完整的SPL Token格式）
        // 这里只是示例，实际项目中需要完整实现
        Ok(TokenAccount {
            mint: "".to_string(), // 需要从数据中解析
            owner: "".to_string(),
            amount: 0,
            decimals: 0,
            delegate: None,
            state: 1,
            is_native: None,
            delegated_amount: 0,
            close_authority: None,
        })
    }

    /// 获取代币账户信息
    pub async fn get_token_account_info(&self, pubkey: &str) -> AppResult<Option<TokenAccount>> {
        let account = self.get_account_info(pubkey).await?;
        
        match account {
            Some(acc) => {
                // 检查是否是SPL Token程序拥有的账户
                if acc.owner != "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" {
                    return Ok(None);
                }

                // 解码Base64数据
                let data = base64::decode(&acc.data[0])
                    .map_err(|e| AppError::BlockchainError(format!("Base64解码失败: {}", e)))?;

                match self.parse_token_account_data(&data) {
                    Ok(token_account) => Ok(Some(token_account)),
                    Err(e) => {
                        warn!("解析代币账户数据失败: {}", e);
                        Ok(None)
                    }
                }
            }
            None => Ok(None),
        }
    }

    /// 获取代币供应量
    pub async fn get_token_supply(&self, mint_pubkey: &str) -> AppResult<u64> {
        let params = json!([
            mint_pubkey,
            {
                "commitment": "confirmed"
            }
        ]);

        let result: Value = self.send_rpc_request("getTokenSupply", params).await?;
        
        let amount_str = result["value"]["amount"].as_str()
            .ok_or_else(|| AppError::BlockchainError("无法获取代币供应量".to_string()))?;
        
        amount_str.parse::<u64>()
            .map_err(|e| AppError::BlockchainError(format!("解析供应量失败: {}", e)))
    }

    /// 获取代币账户余额
    pub async fn get_token_account_balance(&self, token_account: &str) -> AppResult<u64> {
        let params = json!([
            token_account,
            {
                "commitment": "confirmed"
            }
        ]);

        let result: Value = self.send_rpc_request("getTokenAccountBalance", params).await?;
        
        let amount_str = result["value"]["amount"].as_str()
            .ok_or_else(|| AppError::BlockchainError("无法获取代币余额".to_string()))?;
        
        amount_str.parse::<u64>()
            .map_err(|e| AppError::BlockchainError(format!("解析余额失败: {}", e)))
    }

    /// 获取钱包的所有代币账户
    pub async fn get_token_accounts_by_owner(&self, owner: &str) -> AppResult<Vec<(String, TokenAccount)>> {
        let params = json!([
            owner,
            {
                "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            },
            {
                "encoding": "base64",
                "commitment": "confirmed"
            }
        ]);

        let result: Value = self.send_rpc_request("getTokenAccountsByOwner", params).await?;
        
        let mut token_accounts = Vec::new();
        
        if let Some(accounts) = result["value"].as_array() {
            for account in accounts {
                if let Some(pubkey) = account["pubkey"].as_str() {
                    if let Some(account_data) = account["account"]["data"].as_array() {
                        if let Some(data_str) = account_data[0].as_str() {
                            if let Ok(data) = base64::decode(data_str) {
                                if let Ok(token_account) = self.parse_token_account_data(&data) {
                                    token_accounts.push((pubkey.to_string(), token_account));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(token_accounts)
    }

    /// 获取最新区块高度
    pub async fn get_slot(&self) -> AppResult<u64> {
        let params = json!([
            {
                "commitment": "confirmed"
            }
        ]);

        self.send_rpc_request("getSlot", params).await
    }

    /// 检查RPC连接健康状态
    pub async fn health_check(&self) -> AppResult<()> {
        let params = json!([]);
        
        match self.send_rpc_request::<String>("getHealth", params).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("RPC健康检查失败: {}", e);
                Err(AppError::BlockchainError(format!("RPC健康检查失败: {}", e)))
            }
        }
    }

    /// 获取完整的代币信息
    pub async fn get_token_info(&self, mint_pubkey: &str) -> AppResult<Option<TokenInfo>> {
        // 获取代币账户信息
        let account = self.get_account_info(mint_pubkey).await?;
        
        match account {
            Some(acc) => {
                // 检查是否是代币铸造账户
                if acc.owner != "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" {
                    return Ok(None);
                }

                // 获取供应量
                let supply = self.get_token_supply(mint_pubkey).await.unwrap_or(0);
                
                // 简化的代币信息（实际需要解析铸造账户数据）
                Ok(Some(TokenInfo {
                    mint: mint_pubkey.to_string(),
                    supply,
                    decimals: 9, // 默认值，应该从账户数据解析
                    mint_authority: None,
                    freeze_authority: None,
                }))
            }
            None => Ok(None),
        }
    }

    /// 获取交易信息
    pub async fn get_transaction(&self, signature: &str) -> AppResult<Option<Value>> {
        let params = json!([
            signature,
            {
                "encoding": "json",
                "commitment": "confirmed",
                "maxSupportedTransactionVersion": 0
            }
        ]);

        match self.send_rpc_request::<Value>("getTransaction", params).await {
            Ok(result) => Ok(Some(result)),
            Err(_) => Ok(None), // 交易不存在或其他错误
        }
    }
}

// 添加base64依赖的简单实现（避免额外依赖）
