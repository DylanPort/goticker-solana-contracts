use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("9AcKM9kiHLxM22V3QUY6QsxQFysWiE8oLqAUA6PrQRWB");

#[program]
pub mod goticker {
    use super::*;

    pub fn initialize_ticker(
        ctx: Context<InitializeTicker>,
        ticker: String,
        target_url: String,
        description: String,
        contract_address: Option<String>,
    ) -> Result<()> {
        let ticker_account = &mut ctx.accounts.ticker_account;
        
        require!(ticker.len() <= 20, ErrorCode::TickerTooLong);
        require!(target_url.len() <= 200, ErrorCode::UrlTooLong);
        require!(description.len() <= 500, ErrorCode::DescriptionTooLong);
        
        // Transfer registration fee to fee recipient (0.1 SOL)
        let registration_fee = 100_000_000; // 0.1 SOL in lamports
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.owner.to_account_info(),
                to: ctx.accounts.fee_recipient.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, registration_fee)?;
        
        ticker_account.owner = ctx.accounts.owner.key();
        ticker_account.ticker = ticker.clone();
        ticker_account.target_url = target_url;
        ticker_account.description = description;
        ticker_account.contract_address = contract_address;
        ticker_account.is_for_sale = false;
        ticker_account.price = 0;
        ticker_account.created_at = Clock::get()?.unix_timestamp;
        
        msg!("Ticker {} initialized for owner {}", ticker, ctx.accounts.owner.key());
        Ok(())
    }

    pub fn update_ticker(
        ctx: Context<UpdateTicker>,
        target_url: Option<String>,
        description: Option<String>,
        contract_address: Option<String>,
    ) -> Result<()> {
        let ticker_account = &mut ctx.accounts.ticker_account;
        
        if let Some(url) = target_url {
            require!(url.len() <= 200, ErrorCode::UrlTooLong);
            ticker_account.target_url = url;
        }
        
        if let Some(desc) = description {
            require!(desc.len() <= 500, ErrorCode::DescriptionTooLong);
            ticker_account.description = desc;
        }
        
        if let Some(contract) = contract_address {
            ticker_account.contract_address = Some(contract);
        }
        
        msg!("Ticker {} updated", ticker_account.ticker);
        Ok(())
    }

    pub fn list_for_sale(
        ctx: Context<ListForSale>,
        price: u64,
    ) -> Result<()> {
        let ticker_account = &mut ctx.accounts.ticker_account;
        
        ticker_account.is_for_sale = true;
        ticker_account.price = price;
        
        msg!("Ticker {} listed for sale at {} lamports", ticker_account.ticker, price);
        Ok(())
    }

    pub fn buy_ticker(
        ctx: Context<BuyTicker>,
    ) -> Result<()> {
        let ticker_account = &mut ctx.accounts.ticker_account;
        
        require!(ticker_account.is_for_sale, ErrorCode::NotForSale);
        
        // Transfer payment from buyer to current owner
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.owner.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, ticker_account.price)?;
        
        // Transfer ownership
        ticker_account.owner = ctx.accounts.buyer.key();
        ticker_account.is_for_sale = false;
        ticker_account.price = 0;
        
        msg!("Ticker {} sold to {}", ticker_account.ticker, ctx.accounts.buyer.key());
        Ok(())
    }

    pub fn cancel_sale(
        ctx: Context<CancelSale>,
    ) -> Result<()> {
        let ticker_account = &mut ctx.accounts.ticker_account;
        
        ticker_account.is_for_sale = false;
        ticker_account.price = 0;
        
        msg!("Sale cancelled for ticker {}", ticker_account.ticker);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(ticker: String)]
pub struct InitializeTicker<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + TickerAccount::INIT_SPACE,
        seeds = [b"ticker", ticker.as_bytes()],
        bump
    )]
    pub ticker_account: Account<'info, TickerAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    /// CHECK: Fee recipient account
    #[account(mut)]
    pub fee_recipient: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTicker<'info> {
    #[account(
        mut,
        has_one = owner,
    )]
    pub ticker_account: Account<'info, TickerAccount>,
    
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ListForSale<'info> {
    #[account(
        mut,
        has_one = owner,
    )]
    pub ticker_account: Account<'info, TickerAccount>,
    
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct BuyTicker<'info> {
    #[account(
        mut,
        has_one = owner,
    )]
    pub ticker_account: Account<'info, TickerAccount>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK: Current owner of the ticker
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelSale<'info> {
    #[account(
        mut,
        has_one = owner,
    )]
    pub ticker_account: Account<'info, TickerAccount>,
    
    pub owner: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct TickerAccount {
    pub owner: Pubkey,
    #[max_len(20)]
    pub ticker: String,
    #[max_len(200)]
    pub target_url: String,
    #[max_len(500)]
    pub description: String,
    #[max_len(100)]
    pub contract_address: Option<String>,
    pub is_for_sale: bool,
    pub price: u64,
    pub created_at: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Ticker symbol is too long")]
    TickerTooLong,
    #[msg("URL is too long")]
    UrlTooLong,
    #[msg("Description is too long")]
    DescriptionTooLong,
    #[msg("Not authorized to perform this action")]
    NotAuthorized,
    #[msg("Ticker is not for sale")]
    NotForSale,
    #[msg("Insufficient registration fee")]
    InsufficientRegistrationFee,
} 