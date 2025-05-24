pub mod user;
pub mod solana;
pub mod trade;
pub mod data;

use std::sync::Arc;
use crate::services::Services;

pub use user::*;
pub use solana::*;
pub use trade::*;
pub use data::*;

#[derive(Clone)]
pub struct AppState {
    pub services: Arc<Services>,
}

impl AppState {
    pub fn new(services: Arc<Services>) -> Self {
        Self { services }
    }
}
