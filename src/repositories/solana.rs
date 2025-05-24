use sqlx::MySqlPool;
use crate::models::solana::{SolToken, SolTransaction, SolHolder, SolPool, SolStat};
use crate::utils::AppResult;
use chrono::{DateTime, Utc};

pub struct SolanaRepository {
    pool: MySqlPool,
}

impl SolanaRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
    
    // Token相关方法
    pub async fn find_token_by_mint(&self, mint: &str) -> AppResult<Option<SolToken>> {
        let query = r#"
            SELECT id, created_at, updated_at, deleted_at, create_time, ecosystem, mint, creator,
                   symbol, name, decimals, supply, description, logo, banner_url, social,
                   progress, price, price_change, volume, market_cap, holder_count
            FROM cook_wm_sol_token 
            WHERE mint = ? AND deleted_at IS NULL
        "#;
        
        let token = sqlx::query_as::<_, SolToken>(query)
            .bind(mint)
            .fetch_optional(&self.pool)
            .await?;
        
        Ok(token)
    }
    
    pub async fn list_tokens(&self, limit: u32, offset: u32, ecosystem: Option<&str>) -> AppResult<Vec<SolToken>> {
        let mut query = r#"
            SELECT id, created_at, updated_at, deleted_at, create_time, ecosystem, mint, creator,
                   symbol, name, decimals, supply, description, logo, banner_url, social,
                   progress, price, price_change, volume, market_cap, holder_count
            FROM cook_wm_sol_token 
            WHERE deleted_at IS NULL
        "#.to_string();
        
        if ecosystem.is_some() {
            query.push_str(" AND ecosystem = ?");
        }
        
        query.push_str(" ORDER BY market_cap DESC LIMIT ? OFFSET ?");
        
        let mut query_builder = sqlx::query_as::<_, SolToken>(&query);
        
        if let Some(eco) = ecosystem {
            query_builder = query_builder.bind(eco);
        }
        
        let tokens = query_builder
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        
        Ok(tokens)
    }
    
    pub async fn update_token_price(&self, mint: &str, price: f64, market_cap: f64) -> AppResult<()> {
        let query = r#"
            UPDATE cook_wm_sol_token 
            SET price = ?, market_cap = ?, updated_at = NOW()
            WHERE mint = ?
        "#;
        
        sqlx::query(query)
            .bind(price)
            .bind(market_cap)
            .bind(mint)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    // Transaction相关方法
    pub async fn create_transaction(&self, transaction: &SolTransaction) -> AppResult<u32> {
        let query = r#"
            INSERT INTO cook_wm_sol_transaction (
                create_time, mint, curve, signature, signer, token_amount, sol_amount,
                new_price_usd, new_price_sol, is_buy, volume_usd, slot, pnl, transfer_type,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW(), NOW())
        "#;
        
        let result = sqlx::query(query)
            .bind(&transaction.create_time)
            .bind(&transaction.mint)
            .bind(&transaction.curve)
            .bind(&transaction.signature)
            .bind(&transaction.signer)
            .bind(transaction.token_amount)
            .bind(transaction.sol_amount)
            .bind(transaction.new_price_usd)
            .bind(transaction.new_price_sol)
            .bind(transaction.is_buy)
            .bind(transaction.volume_usd)
            .bind(&transaction.slot)
            .bind(transaction.pnl)
            .bind(transaction.transfer_type)
            .execute(&self.pool)
            .await?;
        
        Ok(result.last_insert_id() as u32)
    }
    
    pub async fn find_transactions_by_mint(&self, mint: &str, limit: u32, offset: u32) -> AppResult<Vec<SolTransaction>> {
        let query = r#"
            SELECT id, created_at, updated_at, deleted_at, create_time, mint, curve, signature,
                   signer, token_amount, sol_amount, new_price_usd, new_price_sol, is_buy,
                   volume_usd, slot, pnl, transfer_type
            FROM cook_wm_sol_transaction 
            WHERE mint = ? AND deleted_at IS NULL
            ORDER BY create_time DESC
            LIMIT ? OFFSET ?
        "#;
        
        let transactions = sqlx::query_as::<_, SolTransaction>(query)
            .bind(mint)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        
        Ok(transactions)
    }
    
    pub async fn find_transactions_by_signer(&self, signer: &str, limit: u32, offset: u32) -> AppResult<Vec<SolTransaction>> {
        let query = r#"
            SELECT id, created_at, updated_at, deleted_at, create_time, mint, curve, signature,
                   signer, token_amount, sol_amount, new_price_usd, new_price_sol, is_buy,
                   volume_usd, slot, pnl, transfer_type
            FROM cook_wm_sol_transaction 
            WHERE signer = ? AND deleted_at IS NULL
            ORDER BY create_time DESC
            LIMIT ? OFFSET ?
        "#;
        
        let transactions = sqlx::query_as::<_, SolTransaction>(query)
            .bind(signer)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        
        Ok(transactions)
    }
    
    // Holder相关方法
    pub async fn find_holders_by_mint(&self, mint: &str, limit: u32, offset: u32) -> AppResult<Vec<SolHolder>> {
        let query = r#"
            SELECT id, created_at, updated_at, deleted_at, mint, holder, amount, price_usd, bet, pnl
            FROM cook_wm_sol_holder 
            WHERE mint = ? AND deleted_at IS NULL
            ORDER BY amount DESC
            LIMIT ? OFFSET ?
        "#;
        
        let holders = sqlx::query_as::<_, SolHolder>(query)
            .bind(mint)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        
        Ok(holders)
    }
    
    pub async fn update_holder(&self, mint: &str, holder: &str, amount: f64, price_usd: f64, bet: f64, pnl: f64) -> AppResult<()> {
        let query = r#"
            INSERT INTO cook_wm_sol_holder (mint, holder, amount, price_usd, bet, pnl, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, NOW(), NOW())
            ON DUPLICATE KEY UPDATE
            amount = VALUES(amount),
            price_usd = VALUES(price_usd),
            bet = VALUES(bet),
            pnl = VALUES(pnl),
            updated_at = NOW()
        "#;
        
        sqlx::query(query)
            .bind(mint)
            .bind(holder)
            .bind(amount)
            .bind(price_usd)
            .bind(bet)
            .bind(pnl)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    // Pool相关方法
    pub async fn find_pool_by_curve(&self, curve: &str) -> AppResult<Option<SolPool>> {
        let query = r#"
            SELECT id, created_at, updated_at, deleted_at, ecosystem, curve, mint, reverse_usd,
                   token_reverse, sol_reverse, holder, creator, init_time, decimals
            FROM cook_wm_sol_pool 
            WHERE curve = ? AND deleted_at IS NULL
        "#;
        
        let pool = sqlx::query_as::<_, SolPool>(query)
            .bind(curve)
            .fetch_optional(&self.pool)
            .await?;
        
        Ok(pool)
    }
    
    // 统计相关方法
    pub async fn get_daily_volume(&self, date: DateTime<Utc>) -> AppResult<f64> {
        let query = r#"
            SELECT COALESCE(SUM(volume_usd), 0) as total_volume
            FROM cook_wm_sol_transaction 
            WHERE DATE(create_time) = DATE(?)
        "#;
        
        let result: (f64,) = sqlx::query_as(query)
            .bind(date)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(result.0)
    }
    
    pub async fn get_token_stats(&self, mint: &str, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> AppResult<Option<SolStat>> {
        let query = r#"
            SELECT id, created_at, updated_at, deleted_at, mint, create_time, volume_token,
                   volume_sol, volume_usd, buy_count, sell_count, buy_usdt, sell_usdt, price_usd
            FROM cook_wm_sol_stat 
            WHERE mint = ? AND create_time BETWEEN ? AND ?
            ORDER BY create_time DESC
            LIMIT 1
        "#;
        
        let stat = sqlx::query_as::<_, SolStat>(query)
            .bind(mint)
            .bind(start_time)
            .bind(end_time)
            .fetch_optional(&self.pool)
            .await?;
        
        Ok(stat)
    }
}
