use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("The current stage is not appropriate for trade or cancellation.")]
    InvalidStage,
    #[msg("There are insufficient funds available to proceed.")]
    InsufficientFunds,
    #[msg("The specified mint account for trade is invalid.")]
    InvalidMint,
    #[msg("A mandatory mint for the trade is missing.")]
    MissingMint,
    #[msg("The trade type is invalid, possibly due to missing mint addresses.")]
    InvalidTradeType,
    #[msg("Invalid association between the provided token accounts.")]
    InvalidAccount,
    #[msg("Duplicate mint accounts are not allowed.")]
    DuplicateMint,
    #[msg("The account does not have a valid owner.")]
    InvalidOwner,
    #[msg("The specified partner is not valid for this trade.")]
    InvalidPartner,
    #[msg("Both trade value and receive value must be greater than zero.")]
    ZeroValue,
    #[msg("Necessary parameters for the instruction are missing.")]
    MissingParams,
}
