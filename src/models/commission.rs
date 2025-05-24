use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Commission {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: u32,
    pub referrer_id: u32,
    pub trade_id: u32,
    pub amount: f64,
    pub commission_rate: f64,
    pub commission_amount: f64,
    pub token_type: String, // sol, eth, base
    pub status: String, // pending, paid
}

impl Commission {
    pub fn table_name() -> &'static str {
        "commissions"
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommissionResponse {
    pub id: u32,
    pub user_id: u32,
    pub referrer_id: u32,
    pub amount: f64,
    pub commission_rate: f64,
    pub commission_amount: f64,
    pub token_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<Commission> for CommissionResponse {
    fn from(commission: Commission) -> Self {
        Self {
            id: commission.id,
            user_id: commission.user_id,
            referrer_id: commission.referrer_id,
            amount: commission.amount,
            commission_rate: commission.commission_rate,
            commission_amount: commission.commission_amount,
            token_type: commission.token_type,
            status: commission.status,
            created_at: commission.created_at,
        }
    }
}
