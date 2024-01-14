use anchor_lang::prelude::*;

declare_id!("A3sWyZrffMAYLf84384gvWDvxdyE5YYfWBmbhP2GfuE5");

#[program]
pub mod prediction_market {
    use super::*;

    #[account]
    pub struct MarketState {
        pub locked_price: u64,
        pub closed_price: Option<u64>,
    }

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(init, payer = user, space = 8 + 8)]
        pub market_state: Account<'info, MarketState>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct EnterUpPosition<'info> {
        #[account(mut)]
        pub market_state: Account<'info, MarketState>,
        #[account(mut, signer)]
        pub user: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct EnterDownPosition<'info> {
        #[account(mut)]
        pub market_state: Account<'info, MarketState>,
        #[account(mut, signer)]
        pub user: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct ClosePosition<'info> {
        #[account(mut)]
        pub market_state: Account<'info, MarketState>,
        #[account(mut, signer)]
        pub user: Signer<'info>,
    }

    pub fn initialize(ctx: Context<Initialize>, locked_price: u64) -> Result<()> {
        let market_state = &mut ctx.accounts.market_state;
        market_state.locked_price = locked_price;
        Ok(())
    }

    pub fn enter_up_position(ctx: Context<EnterUpPosition>) -> Result<()> {
        let market_state = &mut ctx.accounts.market_state;
        market_state.locked_price += 1;
        msg!("User entered an UP position. Locked price incremented.");
        Ok(())
    }

    pub fn enter_down_position(ctx: Context<EnterDownPosition>) -> Result<()> {
        let market_state = &mut ctx.accounts.market_state;
        market_state.locked_price -= 1;
        msg!("User entered a DOWN position. Locked price decremented.");
        Ok(())
    }

    pub fn close_position(ctx: Context<ClosePosition>, closed_price: u64) -> Result<()> {
        let market_state = &mut ctx.accounts.market_state;
        market_state.closed_price = Some(closed_price);

        if closed_price > market_state.locked_price {
            msg!("Position is a WIN!");
            
        } else if closed_price < market_state.locked_price {
            msg!("Position is a LOSS!");

        } else {
            msg!("Position is a TIE or UNCHANGED.");
        }

        Ok(())
    }
}
