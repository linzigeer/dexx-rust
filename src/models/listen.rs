use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Listen {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: u32,
    pub mint: String,
    pub is_active: bool,
    pub notification_type: String, // price_alert, volume_alert, etc.
    pub threshold_value: f64,
    pub condition_type: String, // above, below, change_percent
}

impl Listen {
    pub fn table_name() -> &'static str {
        "listens"
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListenRequest {
    pub mint: String,
    pub notification_type: String,
    pub threshold_value: f64,
    pub condition_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenResponse {
    pub id: u32,
    pub mint: String,
    pub is_active: bool,
    pub notification_type: String,
    pub threshold_value: f64,
    pub condition_type: String,
    pub created_at: DateTime<Utc>,
}

impl From<Listen> for ListenResponse {
    fn from(listen: Listen) -> Self {
        Self {
            id: listen.id,
            mint: listen.mint,
            is_active: listen.is_active,
            notification_type: listen.notification_type,
            threshold_value: listen.threshold_value,
            condition_type: listen.condition_type,
            created_at: listen.created_at,
        }
    }
}
