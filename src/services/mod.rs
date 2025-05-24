pub mod user;
pub mod solana;
pub mod trade;
pub mod data;
pub mod message;

use std::sync::Arc;
use crate::config::Config;
use crate::repositories::Repositories;
use crate::sdk;
use crate::utils::AppResult;

pub use user::*;
pub use solana::*;
pub use trade::*;
pub use data::*;
pub use message::*;

pub struct Services {
    pub user: UserService,
    pub solana: SolanaService,
    pub trade: TradeService,
    pub data: DataService,
    pub message: MessageService,
}

impl Services {
    pub async fn new(
        config: Config,
        repositories: Arc<Repositories>,
        solana_sdk: Arc<dyn sdk::SolanaSdk>,
        ethereum_sdk: Arc<dyn sdk::EthereumSdk>,
        gecko_sdk: Arc<dyn sdk::GeckoSdk>,
        binance_sdk: Arc<dyn sdk::BinanceSdk>,
    ) -> AppResult<Self> {
        let message_service = MessageService::new();
        
        Ok(Self {
            user: UserService::new(config.clone(), repositories.clone()),
            solana: SolanaService::new(
                config.clone(),
                repositories.clone(),
                solana_sdk,
                gecko_sdk.clone(),
                binance_sdk,
                message_service.clone(),
            ),
            trade: TradeService::new(
                config.clone(),
                repositories.clone(),
                solana_sdk,
                ethereum_sdk,
                message_service.clone(),
            ),
            data: DataService::new(
                config.clone(),
                repositories.clone(),
                gecko_sdk,
            ),
            message: message_service,
        })
    }
}
