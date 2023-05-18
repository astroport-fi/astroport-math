use cosmwasm_std::{ConversionOverflowError, OverflowError, StdError};
use thiserror::Error;

/// This enum describes pair contract errors
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    ConversionOverflowError(#[from] ConversionOverflowError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),
}
