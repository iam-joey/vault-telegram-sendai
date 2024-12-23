use anchor_lang::prelude::*;

pub mod contexts;
pub mod error;
pub mod state;

pub use contexts::*;
declare_id!("7LvLwwxs3EVajKofaBvVkB4L2YNgGXEj3MbpHzAx2vKq");

#[program]
pub mod vault_telegram {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        seed: u64,
        pool_amount: u64,
        minimum_bet: u64,
        maximum_bet: u64,
    ) -> Result<()> {
        ctx.accounts
            .create_bet(pool_amount, minimum_bet, maximum_bet, seed, &ctx.bumps)
    }

    pub fn join(ctx: Context<Join>, _seed: u64) -> Result<()> {
        ctx.accounts.deposit()
    }

    pub fn resolve(ctx: Context<Resolve>, _seed: u64) -> Result<()> {
        ctx.accounts.resolve()
    }
}
