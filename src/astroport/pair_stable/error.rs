use cosmwasm_std::{CheckedMultiplyRatioError, ConversionOverflowError, OverflowError, StdError};
use thiserror::Error;

// use astroport_circular_buffer::error::BufferError;

use super::consts::MINIMUM_LIQUIDITY_AMOUNT;

/// This enum describes stableswap pair contract errors
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] CheckedMultiplyRatioError),

    #[error("Event of zero transfer")]
    InvalidZeroAmount {},

    #[error("Insufficient amount of liquidity")]
    LiquidityAmountTooSmall {},

    #[error("Initial liquidity must be more than {}", MINIMUM_LIQUIDITY_AMOUNT)]
    MinimumLiquidityAmountError {},
}

impl From<OverflowError> for ContractError {
    fn from(o: OverflowError) -> Self {
        StdError::from(o).into()
    }
}

impl From<ConversionOverflowError> for ContractError {
    fn from(o: ConversionOverflowError) -> Self {
        StdError::from(o).into()
    }
}
