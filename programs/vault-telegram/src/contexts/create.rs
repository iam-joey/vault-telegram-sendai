use anchor_lang::prelude::*;

use crate::state::BetState;

#[derive(Accounts)]
#[instruction(seed:u64,pool_amount: u64, minimum_bet: u64, maximum_bet: u64)]
pub struct Create<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = BetState::get_space(),
        seeds=[b"bet_state",creator.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub bet_state: Box<Account<'info, BetState>>,
    #[account(
        seeds=[b"vault",bet_state.key().as_ref()],
        bump
    )]
    pub vault_pool: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Create<'info> {
    pub fn create_bet(
        &mut self,
        pool_amount: u64,
        minimum_bet: u64,
        maximum_bet: u64,
        seed: u64,
        bumps: &CreateBumps,
    ) -> Result<()> {
        self.bet_state.pool_amount = pool_amount;
        self.bet_state.minimum_bet = minimum_bet;
        self.bet_state.maximum_bet = maximum_bet;
        self.bet_state.creator = *self.creator.key;
        self.bet_state.bumps = bumps.bet_state;
        self.bet_state.is_open = false;
        self.bet_state.seed = seed;
        self.bet_state.vault_pool_bump = bumps.vault_pool;
        Ok(())
    }
}
