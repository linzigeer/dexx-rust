use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenSocial {
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub telegram: Option<String>,
    pub discord: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SolToken {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub create_time: i64,
    pub ecosystem: String,
    pub mint: String,
    pub creator: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub supply: f64,
    pub description: String,
    pub logo: String,
    pub banner_url: String,
    pub social: String, // JSON字符串，需要序列化/反序列化
    pub progress: f64,
    pub price: f64,
    pub price_change: String, // JSON字符串
    pub volume: String, // JSON字符串
    pub market_cap: f64,
    pub holder_count: i32,
}

impl SolToken {
    pub fn table_name() -> &'static str {
        "cook_wm_sol_token"
    }
    
    pub fn get_social(&self) -> Result<TokenSocial, serde_json::Error> {
        if self.social.is_empty() {
            return Ok(TokenSocial {
                twitter: None,
                website: None,
                telegram: None,
                discord: None,
            });
        }
        serde_json::from_str(&self.social)
    }
    
    pub fn set_social(&mut self, social: &TokenSocial) -> Result<(), serde_json::Error> {
        self.social = serde_json::to_string(social)?;
        Ok(())
    }
    
    pub fn get_price_change(&self) -> Result<HashMap<String, f64>, serde_json::Error> {
        if self.price_change.is_empty() {
            return Ok(HashMap::new());
        }
        serde_json::from_str(&self.price_change)
    }
    
    pub fn set_price_change(&mut self, price_change: &HashMap<String, f64>) -> Result<(), serde_json::Error> {
        self.price_change = serde_json::to_string(price_change)?;
        Ok(())
    }
    
    pub fn get_volume(&self) -> Result<HashMap<String, serde_json::Value>, serde_json::Error> {
        if self.volume.is_empty() {
            return Ok(HashMap::new());
        }
        serde_json::from_str(&self.volume)
    }
    
    pub fn set_volume(&mut self, volume: &HashMap<String, serde_json::Value>) -> Result<(), serde_json::Error> {
        self.volume = serde_json::to_string(volume)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SolTransaction {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub mint: String,
    pub curve: String,
    pub signature: String,
    pub signer: String,
    pub token_amount: f64,
    pub sol_amount: f64,
    pub new_price_usd: f64,
    pub new_price_sol: f64,
    pub is_buy: bool,
    pub volume_usd: f64,
    pub slot: String,
    pub pnl: f64,
    pub transfer_type: u8, // 1=建仓2=清仓3=加仓4=减仓
}

impl SolTransaction {
    pub fn table_name() -> &'static str {
        "cook_wm_sol_transaction"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SolHolder {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub mint: String,
    pub holder: String,
    pub amount: f64,
    pub price_usd: f64,
    pub bet: f64,
    pub pnl: f64,
}

impl SolHolder {
    pub fn table_name() -> &'static str {
        "cook_wm_sol_holder"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SolPool {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub ecosystem: String,
    pub curve: String,
    pub mint: String,
    pub reverse_usd: f64,
    pub token_reverse: f64,
    pub sol_reverse: f64,
    pub holder: i64,
    pub creator: String,
    pub init_time: i64,
    pub decimals: u8,
}

impl SolPool {
    pub fn table_name() -> &'static str {
        "cook_wm_sol_pool"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SolStat {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub mint: String,
    pub create_time: DateTime<Utc>,
    pub volume_token: f64,
    pub volume_sol: f64,
    pub volume_usd: f64,
    pub buy_count: u64,
    pub sell_count: u64,
    pub buy_usdt: f64,
    pub sell_usdt: f64,
    pub price_usd: f64,
}

impl SolStat {
    pub fn table_name() -> &'static str {
        "cook_wm_sol_stat"
    }
}

// 用于API响应的简化结构
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub mint: String,
    pub symbol: String,
    pub name: String,
    pub logo: String,
    pub price: f64,
    pub market_cap: f64,
    pub progress: f64,
    pub holder_count: i32,
    pub price_change: HashMap<String, f64>,
    pub volume: HashMap<String, serde_json::Value>,
    pub social: TokenSocial,
}

impl TryFrom<SolToken> for TokenResponse {
    type Error = serde_json::Error;
    
    fn try_from(token: SolToken) -> Result<Self, Self::Error> {
        Ok(Self {
            mint: token.mint,
            symbol: token.symbol,
            name: token.name,
            logo: token.logo,
            price: token.price,
            market_cap: token.market_cap,
            progress: token.progress,
            holder_count: token.holder_count,
            price_change: serde_json::from_str(&token.price_change).unwrap_or_default(),
            volume: serde_json::from_str(&token.volume).unwrap_or_default(),
            social: serde_json::from_str(&token.social).unwrap_or_default(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub signature: String,
    pub mint: String,
    pub signer: String,
    pub token_amount: f64,
    pub sol_amount: f64,
    pub price_usd: f64,
    pub is_buy: bool,
    pub volume_usd: f64,
    pub create_time: DateTime<Utc>,
    pub transfer_type: u8,
}

impl From<SolTransaction> for TransactionResponse {
    fn from(tx: SolTransaction) -> Self {
        Self {
            signature: tx.signature,
            mint: tx.mint,
            signer: tx.signer,
            token_amount: tx.token_amount,
            sol_amount: tx.sol_amount,
            price_usd: tx.new_price_usd,
            is_buy: tx.is_buy,
            volume_usd: tx.volume_usd,
            create_time: tx.create_time,
            transfer_type: tx.transfer_type,
        }
    }
}

// API请求和响应类型
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfoRequest {
    pub mint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPriceRequest {
    pub mint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RankRequest {
    pub sort_by: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenHolderRequest {
    pub mint: String,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeLatestRequest {
    pub mint: String,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletPositionRequest {
    pub wallet: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPositionRequest {
    pub mint: String,
    pub wallet: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiTokenInfoRequest {
    pub mints: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionVolumeRequest {
    pub mint: String,
    pub period: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyTransactionVolumeRequest {
    pub mint: String,
    pub days: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookRequest {
    pub event_type: String,
    pub data: serde_json::Value,
}

// API响应类型
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenHolder {
    pub holder: String,
    pub amount: f64,
    pub percentage: f64,
    pub value_usd: f64,
}

impl From<SolHolder> for TokenHolder {
    fn from(holder: SolHolder) -> Self {
        Self {
            holder: holder.holder,
            amount: holder.amount,
            percentage: 0.0, // 需要计算
            value_usd: holder.amount * holder.price_usd,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub signature: String,
    pub mint: String,
    pub trader: String,
    pub token_amount: f64,
    pub sol_amount: f64,
    pub price_usd: f64,
    pub is_buy: bool,
    pub timestamp: DateTime<Utc>,
}

impl From<SolTransaction> for Trade {
    fn from(tx: SolTransaction) -> Self {
        Self {
            signature: tx.signature,
            mint: tx.mint,
            trader: tx.signer,
            token_amount: tx.token_amount,
            sol_amount: tx.sol_amount,
            price_usd: tx.new_price_usd,
            is_buy: tx.is_buy,
            timestamp: tx.create_time,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub mint: String,
    pub wallet: String,
    pub amount: f64,
    pub value_usd: f64,
    pub pnl: f64,
    pub pnl_percentage: f64,
}

impl From<SolHolder> for Position {
    fn from(holder: SolHolder) -> Self {
        Self {
            mint: holder.mint,
            wallet: holder.holder,
            amount: holder.amount,
            value_usd: holder.amount * holder.price_usd,
            pnl: holder.pnl,
            pnl_percentage: if holder.bet > 0.0 { holder.pnl / holder.bet * 100.0 } else { 0.0 },
        }
    }
}
