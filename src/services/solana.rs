use std::sync::Arc;
use crate::config::Config;
use crate::repositories::RepositoriesImpl;
use crate::models::solana::*;
use crate::utils::{AppResult, AppError};
use serde::{Deserialize, Serialize};

/// 代币查询请求
#[derive(Debug, Deserialize)]
pub struct TokenQueryRequest {
    pub mint: Option<String>,
    pub symbol: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// 代币列表响应
#[derive(Debug, Serialize)]
pub struct TokenListResponse {
    pub tokens: Vec<SolToken>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
}

/// Solana服务实现
pub struct SolanaServiceImpl {
    config: Arc<Config>,
    repositories: Arc<RepositoriesImpl>,
}

impl SolanaServiceImpl {
    /// 创建新的Solana服务实例
    pub async fn new(
        config: Arc<Config>,
        repositories: Arc<RepositoriesImpl>,
    ) -> AppResult<Self> {
        Ok(Self {
            config,
            repositories,
        })
    }

    /// 构建Redis缓存键
    fn build_cache_key(&self, prefix: &str, key: &str) -> String {
        format!("sol:{}:{}", prefix, key)
    }

    /// 根据mint获取代币信息
    pub async fn get_token_by_mint(&self, mint: &str) -> AppResult<Option<SolToken>> {
        // 从数据库获取
        self.repositories.solana_repository().find_token_by_mint(mint).await
    }

    /// 获取代币价格（从数据库）
    pub async fn get_token_price(&self, mint: &str) -> AppResult<Option<f64>> {
        if let Some(token) = self.get_token_by_mint(mint).await? {
            Ok(Some(token.price))
        } else {
            Ok(None)
        }
    }

    /// 更新代币价格 - 简化版本
    pub async fn update_token_price(&self, mint: &str, price: f64, market_cap: f64) -> AppResult<()> {
        self.repositories.solana_repository()
            .update_token_price(mint, price, market_cap).await?;
        Ok(())
    }
}
