use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal256, StdError, StdResult, Uint128};

use super::{
    consts::MINIMUM_LIQUIDITY_AMOUNT,
    error::ContractError,
    math::compute_d,
    state::{compute_current_amp, greatest_precision},
};

use crate::astroport::cosmwasm_ext::Decimal256Ext;

#[cw_serde]
pub struct SimulationResponse {
    /// The amount of lps returned by the provide
    pub share_amount: Uint128,
}

pub fn simulate(
    deposits: &[Decimal256],
    asset_amounts: &[Decimal256],
    asset_precisions: &[u8],
    total_share: Uint128,
    block_time: u64,
    init_amp_time: u64,
    init_amp: u64,
    next_amp_time: u64,
    next_amp: u64,
) -> StdResult<SimulationResponse> {
    let deposits = deposits
        .iter()
        .enumerate()
        .map(|(i, amount)| Decimal256::with_precision(amount.to_uint256(), asset_precisions[i]))
        .collect::<Result<Vec<Decimal256>, StdError>>()?;

    let asset_amounts = asset_amounts
        .iter()
        .enumerate()
        .map(|(i, amount)| Decimal256::with_precision(amount.to_uint256(), asset_precisions[i]))
        .collect::<Result<Vec<Decimal256>, StdError>>()?;

    let share_amount = compute_provide(
        &deposits,
        &asset_amounts,
        asset_precisions,
        total_share,
        block_time,
        init_amp_time,
        init_amp,
        next_amp_time,
        next_amp,
    )
    .map_err(|err| StdError::generic_err(format!("{err}")))?;

    Ok(SimulationResponse { share_amount })
}

fn compute_provide(
    deposits: &[Decimal256],
    asset_amounts: &[Decimal256],
    asset_precisions: &[u8],
    total_share: Uint128,
    block_time: u64,
    init_amp_time: u64,
    init_amp: u64,
    next_amp_time: u64,
    next_amp: u64,
) -> Result<Uint128, ContractError> {
    if deposits[0].is_zero() || deposits[1].is_zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let amp = compute_current_amp(block_time, init_amp_time, init_amp, next_amp_time, next_amp)?;

    let mut non_zero_flag = false;

    deposits.iter().for_each(|amount| {
        // Check that at least one asset is non-zero
        if !amount.is_zero() {
            non_zero_flag = true;
        }
    });

    if !non_zero_flag {
        return Err(ContractError::InvalidZeroAmount {});
    }

    // Invariant (D) after deposit added
    let new_balances = asset_amounts
        .clone()
        .iter()
        .enumerate()
        .map(|(i, amount)| Ok(deposits[i] + amount))
        .collect::<StdResult<Vec<_>>>()?;

    let deposit_d = compute_d(amp, &new_balances)?;

    let share = if total_share.is_zero() {
        let share = deposit_d
            .to_uint128_with_precision(greatest_precision(asset_precisions))?
            .checked_sub(MINIMUM_LIQUIDITY_AMOUNT)
            .map_err(|_| ContractError::MinimumLiquidityAmountError {})?;

        // share cannot become zero after minimum liquidity subtraction
        if share.is_zero() {
            return Err(ContractError::MinimumLiquidityAmountError {});
        }

        share
    } else {
        // Initial invariant (D)
        let init_d = compute_d(amp, &asset_amounts)?;

        let share = Decimal256::with_precision(total_share, greatest_precision(asset_precisions))?
            .checked_multiply_ratio(deposit_d.saturating_sub(init_d), init_d)?
            .to_uint128_with_precision(greatest_precision(asset_precisions))?;

        if share.is_zero() {
            return Err(ContractError::LiquidityAmountTooSmall {});
        }

        share
    };

    Ok(share)
}
