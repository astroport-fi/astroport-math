use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdError, StdResult, Uint128};

use super::{consts::MINIMUM_LIQUIDITY_AMOUNT, error::ContractError};

use crate::astroport::lib::uints::U256;

#[cw_serde]
pub struct SimulationResponse {
    /// The amount of lps returned by the provide
    pub share_amount: Uint128,
}

pub fn simulate(
    deposits: &[Uint128],
    asset_amounts: &[Uint128],
    total_share: Uint128,
) -> StdResult<SimulationResponse> {
    let share_amount = compute_provide(deposits, asset_amounts, total_share)
        .map_err(|err| StdError::generic_err(format!("{err}")))?;

    Ok(SimulationResponse { share_amount })
}

fn compute_provide(
    deposits: &[Uint128],
    asset_amounts: &[Uint128],
    total_share: Uint128,
) -> Result<Uint128, ContractError> {
    if deposits[0].is_zero() || deposits[1].is_zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let share = if total_share.is_zero() {
        // Initial share = collateral amount
        let share = Uint128::new(
            (U256::from(deposits[0].u128()) * U256::from(deposits[1].u128()))
                .integer_sqrt()
                .as_u128(),
        )
        .checked_sub(MINIMUM_LIQUIDITY_AMOUNT)
        .map_err(|_| ContractError::MinimumLiquidityAmountError {})?;

        // share cannot become zero after minimum liquidity subtraction
        if share.is_zero() {
            return Err(ContractError::MinimumLiquidityAmountError {});
        }

        share
    } else {
        // min(1, 2)
        // 1. sqrt(deposit_0 * exchange_rate_0_to_1 * deposit_0) * (total_share / sqrt(pool_0 * pool_0))
        // == deposit_0 * total_share / pool_0
        // 2. sqrt(deposit_1 * exchange_rate_1_to_0 * deposit_1) * (total_share / sqrt(pool_1 * pool_1))
        // == deposit_1 * total_share / pool_1
        std::cmp::min(
            deposits[0].multiply_ratio(total_share, asset_amounts[0]),
            deposits[1].multiply_ratio(total_share, asset_amounts[1]),
        )
    };

    Ok(share)
}
