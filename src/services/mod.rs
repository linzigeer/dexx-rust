use std::sync::Arc;
use crate::config::Config;
use crate::repositories::RepositoriesImpl;
use crate::utils::AppResult;

pub mod user;
pub mod solana;

pub use user::*;
pub use solana::*;

/// 服务层实现
pub struct ServicesImpl {
    user_service: UserServiceImpl,
    solana_service: SolanaServiceImpl,
}

impl ServicesImpl {
    /// 创建新的服务层实例
    pub async fn new(
        config: Arc<Config>,
        repositories: Arc<RepositoriesImpl>,
    ) -> AppResult<Arc<Self>> {
        // 创建用户服务
        let user_service = UserServiceImpl::new(
            config.clone(),
            repositories.clone(),
        ).await?;

        // 创建Solana服务
        let solana_service = SolanaServiceImpl::new(
            config.clone(),
            repositories.clone(),
        ).await?;

        let services = Self {
            user_service,
            solana_service,
        };

        Ok(Arc::new(services))
    }

    pub fn user_service(&self) -> &UserServiceImpl {
        &self.user_service
    }

    pub fn solana_service(&self) -> &SolanaServiceImpl {
        &self.solana_service
    }
}
