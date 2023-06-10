use anchor_lang::error_code;

#[error_code]
pub enum AmmError {
    #[msg("Default Error")]
    DefaultError,
    #[msg("AddLiquiditi has expired")]
    AddLiquidityExpired,
    #[msg("RemoveLiquidity has expired")]
    RemoveLiquidityExpired,
    #[msg("swap has expired")]
    SwapExpired,
    #[msg("Pool is frozen")]
    PoolFrozen,
    #[msg("ammount is let than minimum")]
    AmountLessThanMinimum,
    #[msg("liquidity is less than minimum")]
    LiquidityLessThanMinimum,
    #[msg("no liqudity in the pool")]
    NoLiquidityPoll,
    #[msg("Bump Error")]
    BumpError,
}
