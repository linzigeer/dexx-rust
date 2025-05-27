//! 代币价格获取服务
//! 
//! 从多个数据源获取代币价格信息

use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::{interval, Interval};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::utils::{AppResult, AppError};
use tracing::{info, warn, error, debug};

/// 价格数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub mint: String,
    pub price_usd: f64,
    pub price_change_24h: f64,
    pub volume_24h: f64,
    pub market_cap: f64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Jupiter价格API响应
#[derive(Debug, Deserialize)]
struct JupiterPriceResponse {
    data: HashMap<String, JupiterTokenPrice>,
}

#[derive(Debug, Deserialize)]
struct JupiterTokenPrice {
    id: String,
    #[serde(rename = "mintSymbol")]
    mint_symbol: Option<String>,
    #[serde(rename = "vsToken")]
    vs_token: String,
    #[serde(rename = "vsTokenSymbol")]
    vs_token_symbol: String,
    price: f64,
}

/// CoinGecko价格API响应
#[derive(Debug, Deserialize)]
struct CoinGeckoResponse {
    #[serde(flatten)]
    prices: HashMap<String, CoinGeckoPrice>,
}

#[derive(Debug, Deserialize)]
struct CoinGeckoPrice {
    usd: f64,
    usd_24h_change: Option<f64>,
    usd_market_cap: Option<f64>,
    usd_24h_vol: Option<f64>,
}

/// 价格服务
pub struct PriceService {
    config: Arc<Config>,
    http_client: Client,
    price_cache: Arc<RwLock<HashMap<String, PriceData>>>,
    update_interval: Option<Interval>,
}

impl PriceService {
    /// 创建新的价格服务
    pub async fn new(config: Arc<Config>) -> AppResult<Self> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::BlockchainError(format!("创建HTTP客户端失败: {}", e)))?;

        Ok(Self {
            config,
            http_client,
            price_cache: Arc::new(RwLock::new(HashMap::new())),
            update_interval: None,
        })
    }

    /// 从Jupiter获取价格
    async fn fetch_jupiter_prices(&self, mints: &[String]) -> AppResult<HashMap<String, PriceData>> {
        let mut prices = HashMap::new();
        
        // Jupiter API每次最多查询100个代币
        for chunk in mints.chunks(100) {
            let ids = chunk.join(",");
            let url = format!("https://price.jup.ag/v4/price?ids={}", ids);
            
            debug!("从Jupiter获取价格: {}", url);
            
            match self.http_client.get(&url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<JupiterPriceResponse>().await {
                            Ok(jupiter_response) => {
                                for (mint, price_info) in jupiter_response.data {
                                    let price_data = PriceData {
                                        mint: mint.clone(),
                                        price_usd: price_info.price,
                                        price_change_24h: 0.0, // Jupiter不提供24h变化
                                        volume_24h: 0.0,
                                        market_cap: 0.0,
                                        last_updated: chrono::Utc::now(),
                                    };
                                    prices.insert(mint, price_data);
                                }
                            }
                            Err(e) => warn!("解析Jupiter响应失败: {}", e),
                        }
                    } else {
                        warn!("Jupiter API请求失败: {}", response.status());
                    }
                }
                Err(e) => warn!("Jupiter API请求错误: {}", e),
            }
            
            // 避免请求过于频繁
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        Ok(prices)
    }

    /// 从CoinGecko获取价格（备用数据源）
    async fn fetch_coingecko_prices(&self, coin_ids: &[String]) -> AppResult<HashMap<String, PriceData>> {
        let mut prices = HashMap::new();
        
        // CoinGecko API每次最多查询250个代币
        for chunk in coin_ids.chunks(250) {
            let ids = chunk.join(",");
            let url = format!(
                "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true&include_market_cap=true&include_24hr_vol=true",
                ids
            );
            
            debug!("从CoinGecko获取价格: {}", url);
            
            match self.http_client.get(&url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<CoinGeckoResponse>().await {
                            Ok(cg_response) => {
                                for (coin_id, price_info) in cg_response.prices {
                                    let price_data = PriceData {
                                        mint: coin_id.clone(),
                                        price_usd: price_info.usd,
                                        price_change_24h: price_info.usd_24h_change.unwrap_or(0.0),
                                        volume_24h: price_info.usd_24h_vol.unwrap_or(0.0),
                                        market_cap: price_info.usd_market_cap.unwrap_or(0.0),
                                        last_updated: chrono::Utc::now(),
                                    };
                                    prices.insert(coin_id, price_data);
                                }
                            }
                            Err(e) => warn!("解析CoinGecko响应失败: {}", e),
                        }
                    } else {
                        warn!("CoinGecko API请求失败: {}", response.status());
                    }
                }
                Err(e) => warn!("CoinGecko API请求错误: {}", e),
            }
            
            // CoinGecko有更严格的限流
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        
        Ok(prices)
    }

    /// 获取单个代币价格
    pub async fn get_token_price(&self, mint: &str) -> AppResult<Option<PriceData>> {
        // 首先检查缓存
        {
            let cache = self.price_cache.read().await;
            if let Some(price_data) = cache.get(mint) {
                // 检查数据是否过期（5分钟）
                let age = chrono::Utc::now() - price_data.last_updated;
                if age.num_minutes() < 5 {
                    return Ok(Some(price_data.clone()));
                }
            }
        }

        // 缓存中没有或已过期，从API获取
        let prices = self.fetch_jupiter_prices(&[mint.to_string()]).await?;
        
        if let Some(price_data) = prices.get(mint) {
            // 更新缓存
            {
                let mut cache = self.price_cache.write().await;
                cache.insert(mint.to_string(), price_data.clone());
            }
            Ok(Some(price_data.clone()))
        } else {
            Ok(None)
        }
    }

    /// 批量获取代币价格
    pub async fn get_multiple_token_prices(&self, mints: &[String]) -> AppResult<HashMap<String, PriceData>> {
        let mut result = HashMap::new();
        let mut missing_mints = Vec::new();

        // 检查缓存
        {
            let cache = self.price_cache.read().await;
            for mint in mints {
                if let Some(price_data) = cache.get(mint) {
                    let age = chrono::Utc::now() - price_data.last_updated;
                    if age.num_minutes() < 5 {
                        result.insert(mint.clone(), price_data.clone());
                    } else {
                        missing_mints.push(mint.clone());
                    }
                } else {
                    missing_mints.push(mint.clone());
                }
            }
        }

        // 获取缺失的价格数据
        if !missing_mints.is_empty() {
            let new_prices = self.fetch_jupiter_prices(&missing_mints).await?;
            
            // 更新缓存和结果
            {
                let mut cache = self.price_cache.write().await;
                for (mint, price_data) in new_prices {
                    cache.insert(mint.clone(), price_data.clone());
                    result.insert(mint, price_data);
                }
            }
        }

        Ok(result)
    }

    /// 启动价格更新服务
    pub async fn start_price_updates(&self) -> AppResult<()> {
        info!("启动价格更新服务");
        
        // 这里可以实现定期更新逻辑
        // 例如每5分钟更新一次热门代币的价格
        
        Ok(())
    }

    /// 停止价格更新服务
    pub async fn stop_price_updates(&self) -> AppResult<()> {
        info!("停止价格更新服务");
        Ok(())
    }

    /// 清理过期的价格缓存
    pub async fn cleanup_expired_cache(&self) {
        let mut cache = self.price_cache.write().await;
        let now = chrono::Utc::now();
        
        cache.retain(|_, price_data| {
            let age = now - price_data.last_updated;
            age.num_hours() < 1 // 保留1小时内的数据
        });
        
        debug!("清理过期缓存，剩余 {} 条记录", cache.len());
    }

    /// 获取缓存统计信息
    pub async fn get_cache_stats(&self) -> (usize, usize) {
        let cache = self.price_cache.read().await;
        let total = cache.len();
        let now = chrono::Utc::now();
        
        let fresh = cache.values().filter(|price_data| {
            let age = now - price_data.last_updated;
            age.num_minutes() < 5
        }).count();
        
        (total, fresh)
    }
}
