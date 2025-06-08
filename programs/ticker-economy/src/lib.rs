use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("6qNwVB7W2Mgb2AMB74cxKz9u5q3nf3dyPndHNN2y3Hna"); // Placeholder - Solana Playground will generate new ID

#[program]
pub mod ticker_economy {
    use super::*;

    pub fn initialize_payment_config(
        ctx: Context<InitializePaymentConfig>,
        ticker: String,
        base_fee: u64,
        revenue_shares: Vec<RevenueShare>,
        subscription_enabled: bool,
        subscription_price: u64,
        subscription_period: u64,
    ) -> Result<()> {
        let payment_config = &mut ctx.accounts.payment_config;
        
        payment_config.ticker = ticker;
        payment_config.owner = ctx.accounts.owner.key();
        payment_config.base_fee = base_fee;
        payment_config.revenue_shares = revenue_shares;
        payment_config.subscription_enabled = subscription_enabled;
        payment_config.subscription_price = subscription_price;
        payment_config.subscription_period = subscription_period;
        payment_config.total_payments = 0;
        payment_config.total_revenue = 0;
        payment_config.created_at = Clock::get()?.unix_timestamp;
        
        msg!("Payment config initialized for ticker: {}", payment_config.ticker);
        Ok(())
    }

    pub fn update_payment_config(
        ctx: Context<UpdatePaymentConfig>,
        base_fee: Option<u64>,
        revenue_shares: Option<Vec<RevenueShare>>,
        subscription_enabled: Option<bool>,
        subscription_price: Option<u64>,
        subscription_period: Option<u64>,
    ) -> Result<()> {
        let payment_config = &mut ctx.accounts.payment_config;
        
        if let Some(fee) = base_fee {
            payment_config.base_fee = fee;
        }
        
        if let Some(shares) = revenue_shares {
            payment_config.revenue_shares = shares;
        }
        
        if let Some(enabled) = subscription_enabled {
            payment_config.subscription_enabled = enabled;
        }
        
        if let Some(price) = subscription_price {
            payment_config.subscription_price = price;
        }
        
        if let Some(period) = subscription_period {
            payment_config.subscription_period = period;
        }
        
        msg!("Payment config updated for ticker: {}", payment_config.ticker);
        Ok(())
    }

    pub fn process_payment(
        ctx: Context<ProcessPayment>,
        ticker: String,
        amount: u64,
    ) -> Result<()> {
        let payment_config = &mut ctx.accounts.payment_config;
        let payment_record = &mut ctx.accounts.payment_record;
        
        // Transfer SOL from payer to owner
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.owner.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, amount)?;
        
        // Update payment config stats
        payment_config.total_payments += 1;
        payment_config.total_revenue += amount;
        
        // Create payment record
        payment_record.ticker = ticker;
        payment_record.payer = ctx.accounts.payer.key();
        payment_record.amount = amount;
        payment_record.timestamp = Clock::get()?.unix_timestamp;
        
        msg!("Payment processed: {} lamports for ticker: {}", amount, payment_record.ticker);
        Ok(())
    }

    pub fn create_subscription(
        ctx: Context<CreateSubscription>,
        ticker: String,
    ) -> Result<()> {
        // Get the payment config key before taking mutable borrow
        let payment_config_key = ctx.accounts.payment_config.key();
        
        let payment_config = &mut ctx.accounts.payment_config;
        let subscription = &mut ctx.accounts.subscription;
        let payment_record = &mut ctx.accounts.payment_record;
        
        require!(payment_config.subscription_enabled, ErrorCode::SubscriptionsNotEnabled);
        
        // Transfer subscription payment from subscriber to owner
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.subscriber.to_account_info(),
                to: ctx.accounts.owner.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, payment_config.subscription_price)?;
        
        // Create subscription record
        let current_time = Clock::get()?.unix_timestamp;
        subscription.payment_config = payment_config_key;
        subscription.subscriber = ctx.accounts.subscriber.key();
        subscription.is_active = true;
        subscription.created_at = current_time;
        subscription.expires_at = current_time + payment_config.subscription_period as i64;
        
        // Update payment config stats
        payment_config.total_payments += 1;
        payment_config.total_revenue += payment_config.subscription_price;
        
        // Create payment record
        payment_record.ticker = ticker;
        payment_record.payer = ctx.accounts.subscriber.key();
        payment_record.amount = payment_config.subscription_price;
        payment_record.timestamp = current_time;
        
        msg!("Subscription created for ticker: {} by {}", payment_record.ticker, ctx.accounts.subscriber.key());
        Ok(())
    }

    pub fn renew_subscription(
        ctx: Context<RenewSubscription>,
    ) -> Result<()> {
        let payment_config = &mut ctx.accounts.payment_config;
        let subscription = &mut ctx.accounts.subscription;
        let payment_record = &mut ctx.accounts.payment_record;
        
        require!(payment_config.subscription_enabled, ErrorCode::SubscriptionsNotEnabled);
        require!(subscription.is_active, ErrorCode::SubscriptionNotActive);
        
        // Transfer renewal payment from subscriber to owner
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.subscriber.to_account_info(),
                to: ctx.accounts.owner.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, payment_config.subscription_price)?;
        
        // Extend subscription
        let current_time = Clock::get()?.unix_timestamp;
        subscription.expires_at = std::cmp::max(subscription.expires_at, current_time) + payment_config.subscription_period as i64;
        
        // Update payment config stats
        payment_config.total_payments += 1;
        payment_config.total_revenue += payment_config.subscription_price;
        
        // Create payment record
        payment_record.ticker = payment_config.ticker.clone();
        payment_record.payer = ctx.accounts.subscriber.key();
        payment_record.amount = payment_config.subscription_price;
        payment_record.timestamp = current_time;
        
        msg!("Subscription renewed for {}", ctx.accounts.subscriber.key());
        Ok(())
    }

    pub fn cancel_subscription(
        ctx: Context<CancelSubscription>,
    ) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        
        subscription.is_active = false;
        
        msg!("Subscription cancelled for {}", ctx.accounts.subscriber.key());
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(ticker: String)]
pub struct InitializePaymentConfig<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 24 + 32 + 8 + 334 + 1 + 8 + 8 + 8 + 8 + 8, // 437 bytes total
        seeds = [b"payment-config", ticker.as_bytes()],
        bump
    )]
    pub payment_config: Account<'info, PaymentConfig>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePaymentConfig<'info> {
    #[account(
        mut,
        has_one = owner,
    )]
    pub payment_config: Account<'info, PaymentConfig>,
    
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(ticker: String)]
pub struct ProcessPayment<'info> {
    #[account(
        mut,
        seeds = [b"payment-config", ticker.as_bytes()],
        bump
    )]
    pub payment_config: Account<'info, PaymentConfig>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: This is the payment config owner
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + 24 + 32 + 8 + 8 // 80 bytes total
    )]
    pub payment_record: Account<'info, PaymentRecord>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(ticker: String)]
pub struct CreateSubscription<'info> {
    #[account(
        mut,
        seeds = [b"payment-config", ticker.as_bytes()],
        bump
    )]
    pub payment_config: Account<'info, PaymentConfig>,
    
    #[account(mut)]
    pub subscriber: Signer<'info>,
    
    /// CHECK: This is the payment config owner
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    
    #[account(
        init,
        payer = subscriber,
        space = 8 + 32 + 32 + 1 + 8 + 8, // 89 bytes total
        seeds = [b"subscription", payment_config.key().as_ref(), subscriber.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, Subscription>,
    
    #[account(
        init,
        payer = subscriber,
        space = 8 + 24 + 32 + 8 + 8 // 80 bytes total
    )]
    pub payment_record: Account<'info, PaymentRecord>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RenewSubscription<'info> {
    #[account(mut)]
    pub payment_config: Account<'info, PaymentConfig>,
    
    #[account(mut)]
    pub subscriber: Signer<'info>,
    
    /// CHECK: This is the payment config owner
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    
    #[account(
        mut,
        has_one = subscriber,
        seeds = [b"subscription", payment_config.key().as_ref(), subscriber.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, Subscription>,
    
    #[account(
        init,
        payer = subscriber,
        space = 8 + 24 + 32 + 8 + 8 // 80 bytes total
    )]
    pub payment_record: Account<'info, PaymentRecord>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelSubscription<'info> {
    #[account(
        mut,
        has_one = subscriber,
    )]
    pub subscription: Account<'info, Subscription>,
    
    pub subscriber: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PaymentConfig {
    pub ticker: String,              // 4 + 20 = 24 bytes
    pub owner: Pubkey,               // 32 bytes
    pub base_fee: u64,               // 8 bytes
    pub revenue_shares: Vec<RevenueShare>, // 4 + (10 * 33) = 334 bytes
    pub subscription_enabled: bool,  // 1 byte
    pub subscription_price: u64,     // 8 bytes
    pub subscription_period: u64,    // 8 bytes
    pub total_payments: u64,         // 8 bytes
    pub total_revenue: u64,          // 8 bytes
    pub created_at: i64,             // 8 bytes
}

#[account]
pub struct Subscription {
    pub payment_config: Pubkey,      // 32 bytes
    pub subscriber: Pubkey,          // 32 bytes
    pub is_active: bool,             // 1 byte
    pub created_at: i64,             // 8 bytes
    pub expires_at: i64,             // 8 bytes
}

#[account]
pub struct PaymentRecord {
    pub ticker: String,              // 4 + 20 = 24 bytes
    pub payer: Pubkey,               // 32 bytes
    pub amount: u64,                 // 8 bytes
    pub timestamp: i64,              // 8 bytes
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RevenueShare {
    pub recipient: Pubkey,           // 32 bytes
    pub percentage: u8,              // 1 byte
}

#[error_code]
pub enum ErrorCode {
    #[msg("Subscriptions are not enabled for this ticker")]
    SubscriptionsNotEnabled,
    #[msg("Subscription is not active")]
    SubscriptionNotActive,
} 