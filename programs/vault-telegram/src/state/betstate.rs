use anchor_lang::prelude::*;

#[account]
pub struct BetState {
    pub creator: Pubkey,
    pub pool_amount: u64, //in sol amount eg:1 SOL or 2 SOL
    pub minimum_bet: u64, //store in lamports
    pub maximum_bet: u64, // store in lamports
    pub total_bets: u64,  //total users to join
    pub is_open: bool,
    pub users: Vec<Pubkey>,
    pub seed: u64,
    pub bumps: u8,
    pub vault_pool_bump: u8,
}

impl BetState {
    pub fn get_space() -> usize {
        let pubkey_size = 32;
        let u64_size = 8;
        let bool_size = 1;
        let vec_size = 4 + 10 * pubkey_size;
        let u8_size = 1;

        8 + 
        pubkey_size +
        u64_size +
        u64_size +
        u64_size +
        u64_size +
        bool_size +
        vec_size +
        u64_size +
        u8_size +
        u8_size
    }
}