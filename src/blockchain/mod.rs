//! 区块链集成模块
//! 
//! 提供Solana区块链的完整集成功能，包括：
//! - RPC客户端
//! - 交易监听
//! - 代币价格获取
//! - 钱包签名验证

pub mod solana_client;
pub mod price_service;
pub mod transaction_listener;
pub mod wallet_verifier;

pub use solana_client::*;
pub use price_service::*;
pub use transaction_listener::*;
pub use wallet_verifier::*;

use std::sync::Arc;
use crate::config::Config;
use crate::utils::AppResult;

/// 区块链服务集合
#[derive(Clone)]
pub struct BlockchainServices {
    pub solana_client: Arc<SolanaClientService>,
    pub price_service: Arc<PriceService>,
    pub transaction_listener: Arc<TransactionListener>,
    pub wallet_verifier: Arc<WalletVerifier>,
}

impl BlockchainServices {
    /// 创建新的区块链服务实例
    pub async fn new(config: Arc<Config>) -> AppResult<Self> {
        let solana_client = Arc::new(SolanaClientService::new(config.clone()).await?);
        let price_service = Arc::new(PriceService::new(config.clone()).await?);
        let transaction_listener = Arc::new(TransactionListener::new(config.clone()).await?);
        let wallet_verifier = Arc::new(WalletVerifier::new(config.clone())?);

        Ok(Self {
            solana_client,
            price_service,
            transaction_listener,
            wallet_verifier,
        })
    }

    /// 启动所有后台服务
    pub async fn start_background_services(&self) -> AppResult<()> {
        // 启动价格更新服务
        self.price_service.start_price_updates().await?;
        
        // 启动交易监听服务
        self.transaction_listener.start_listening().await?;
        
        Ok(())
    }

    /// 停止所有后台服务
    pub async fn stop_background_services(&self) -> AppResult<()> {
        self.price_service.stop_price_updates().await?;
        self.transaction_listener.stop_listening().await?;
        Ok(())
    }
}
