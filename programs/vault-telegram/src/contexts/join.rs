use anchor_lang::prelude::*;

use anchor_lang::system_program::{transfer, Transfer};

use crate::error::Errors;
use crate::state::BetState;
#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Join<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: BET MAKER ADDRESS FOR DERIVING PDA
    pub maker: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds=[b"bet_state",maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump=bet_state.bumps
        )]
    pub bet_state: Box<Account<'info, BetState>>,
    #[account(
        mut,
        seeds=[b"vault",bet_state.key().as_ref()],
        bump=bet_state.vault_pool_bump
    )]
    pub vault_pool: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Join<'info> {
    pub fn deposit(&mut self) -> Result<()> {
        self.bet_state.users.push(self.user.key());
        require!(
            self.bet_state.total_bets < self.bet_state.users.len() as u64,
            Errors::PoolFilled
        );
        require!(
            self.user.lamports() > self.bet_state.minimum_bet,
            Errors::InsufficientBalance
        );

        let accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault_pool.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, self.bet_state.minimum_bet)
    }
}
