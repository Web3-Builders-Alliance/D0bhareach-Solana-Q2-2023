use anchor_lang::error_code;

#[error_code]
pub enum EscrowError {
    #[msg("Unable to get auth bump")]
    AuthBump,
    #[msg("Unable to get vault bump")]
    VaultBump,
    #[msg("Unable to get escrow bump")]
    EscrowBump,
    #[msg("Escrow expiration is too fat in the future")]
    MaxExpiryExceeded,
    #[msg("Escrow has expired")]
    Expired,
    #[msg("Expiry error")]
    ExpiryError,
}
