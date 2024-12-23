use anchor_lang::{
    prelude::*, system_program::{transfer, Transfer}
};

use crate::state::BetState;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Resolve<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        seeds = [b"bet_state", creator.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = bet_state.bumps
    )]
    pub bet_state: Box<Account<'info, BetState>>,
    /// CHECK: BET MAKER ADDRESS FOR DERIVING PDA
    pub winner: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", bet_state.key().as_ref()],
        bump = bet_state.vault_pool_bump
    )]
    pub vault_pool: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Resolve<'info> {
    pub fn resolve(&mut self) -> Result<()> {
        let amount = self.vault_pool.lamports();
        let accounts = Transfer {
            from: self.vault_pool.to_account_info(),
            to: self.winner.to_account_info(),
        };
        let binding_key = self.bet_state.key();
        let bump_binding = [self.bet_state.vault_pool_bump];
        let signer_seeds = &[&[b"vault", binding_key.as_ref(), &bump_binding][..]];
        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer(ctx, amount)
    }
}
