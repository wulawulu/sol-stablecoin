use anchor_lang::prelude::*;


#[error_code]
pub enum ErrorCode {
    #[msg("Below minimum health factor.")]
    BelowMinimumHealthFactor,
    #[msg("Invalid price from price feed.")]
    InvalidPrice,
}