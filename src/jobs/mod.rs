pub mod solana;
pub mod trade;
pub mod cook;

use std::sync::Arc;
use crate::config::Config;
use crate::services::Services;
use crate::sdk;
use crate::utils::AppResult;

pub use solana::*;
pub use trade::*;
pub use cook::*;

pub struct Jobs {
    pub solana: SolanaJob,
    pub trade: TradeJob,
    pub cook: CookJob,
}

impl Jobs {
    pub fn new(
        config: Config,
        services: Arc<Services>,
        solana_sdk: Arc<dyn sdk::SolanaSdk>,
        ethereum_sdk: Arc<dyn sdk::EthereumSdk>,
    ) -> Self {
        Self {
            solana: SolanaJob::new(config.clone(), services.clone(), solana_sdk),
            trade: TradeJob::new(config.clone(), services.clone(), ethereum_sdk),
            cook: CookJob::new(config, services),
        }
    }
    
    pub async fn start_all(&self) -> AppResult<()> {
        // 启动所有后台任务
        tokio::spawn(async move {
            // 这里启动各种后台任务
        });
        
        Ok(())
    }
}
