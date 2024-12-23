use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Pool is already full")]
    PoolFilled,
    #[msg("Please fund your account")]
    InsufficientBalance,
}
