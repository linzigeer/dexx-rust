pub mod database;
pub mod redis_repo;
pub mod user;
pub mod solana;
pub mod trade;
pub mod commission;
pub mod listen;

use crate::config::Config;
use crate::utils::AppResult;

pub use database::*;
pub use redis_repo::*;
pub use user::*;
pub use solana::*;
pub use trade::*;
pub use commission::*;
pub use listen::*;

pub struct Repositories {
    pub user: UserRepository,
    pub solana: SolanaRepository,
    pub trade: TradeRepository,
    pub commission: CommissionRepository,
    pub listen: ListenRepository,
    pub redis: RedisRepository,
}

impl Repositories {
    pub async fn new(config: &Config) -> AppResult<Self> {
        let database = create_pool(config).await?;
        let redis = RedisRepository::new(config).await?;
        
        Ok(Self {
            user: UserRepository::new(database.clone()),
            solana: SolanaRepository::new(database.clone()),
            trade: TradeRepository::new(database.clone()),
            commission: CommissionRepository::new(database.clone()),
            listen: ListenRepository::new(database.clone()),
            redis,
        })
    }
}
