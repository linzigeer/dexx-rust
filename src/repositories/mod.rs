pub mod database;
pub mod redis_repo;
pub mod user;
pub mod solana;
pub mod trade;
pub mod commission;
pub mod listen;

use std::sync::Arc;
use crate::config::Config;
use crate::utils::AppResult;

pub use database::*;
pub use redis_repo::*;
pub use user::*;
pub use solana::*;
pub use trade::*;
pub use commission::*;
pub use listen::*;

pub struct RepositoriesImpl {
    pub user: UserRepository,
    pub solana: SolanaRepository,
    pub trade: TradeRepository,
    pub commission: CommissionRepository,
    pub listen: ListenRepository,
    pub redis: RedisRepository,
}

impl RepositoriesImpl {
    pub async fn new(config: Arc<Config>) -> AppResult<Arc<Self>> {
        let database = create_pool(&config).await?;
        let redis = RedisRepository::new(&config).await?;
        
        let repositories = Self {
            user: UserRepository::new(database.clone()),
            solana: SolanaRepository::new(database.clone()),
            trade: TradeRepository::new(database.clone()),
            commission: CommissionRepository::new(database.clone()),
            listen: ListenRepository::new(database.clone()),
            redis,
        };
        
        Ok(Arc::new(repositories))
    }

    pub fn user_repository(&self) -> &UserRepository {
        &self.user
    }

    pub fn solana_repository(&self) -> &SolanaRepository {
        &self.solana
    }

    pub fn trade_repository(&self) -> &TradeRepository {
        &self.trade
    }

    pub fn commission_repository(&self) -> &CommissionRepository {
        &self.commission
    }

    pub fn listen_repository(&self) -> &ListenRepository {
        &self.listen
    }

    pub fn redis_repository(&self) -> &RedisRepository {
        &self.redis
    }
}
