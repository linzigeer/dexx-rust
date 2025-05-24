use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Trade {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: u32,
    pub mint: String,
    pub signature: String,
    pub amount: f64,
    pub price: f64,
    pub is_buy: bool,
    pub status: String, // pending, completed, failed
}

impl Trade {
    pub fn table_name() -> &'static str {
        "trades"
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTradeRequest {
    pub mint: String,
    pub amount: f64,
    pub is_buy: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeResponse {
    pub id: u32,
    pub mint: String,
    pub signature: String,
    pub amount: f64,
    pub price: f64,
    pub is_buy: bool,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<Trade> for TradeResponse {
    fn from(trade: Trade) -> Self {
        Self {
            id: trade.id,
            mint: trade.mint,
            signature: trade.signature,
            amount: trade.amount,
            price: trade.price,
            is_buy: trade.is_buy,
            status: trade.status,
            created_at: trade.created_at,
        }
    }
}
