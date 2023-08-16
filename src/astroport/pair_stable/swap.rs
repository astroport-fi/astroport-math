use crate::astroport::cosmwasm_ext::Decimal256Ext;
use crate::astroport::lib::DecimalCheckedOps;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Decimal256, StdError, StdResult, Uint128};

use super::{error::ContractError, math::calc_y, state::compute_current_amp};

#[cw_serde]
pub struct SimulationResponse {
    /// The amount of ask assets returned by the swap
    pub return_amount: Uint128,
    /// The spread used in the swap operation
    pub spread_amount: Uint128,
    /// The amount of fees charged by the transaction
    pub commission_amount: Uint128,
}

pub fn simulate(
    offer_amount: Decimal256,
    offer_asset_prec: u8,
    ask_ind: usize,
    ask_asset_prec: u8,
    asset_amounts: &[Decimal256],
    total_fee_rate: Decimal,
    block_time: u64,
    init_amp_time: u64,
    init_amp: u64,
    next_amp_time: u64,
    next_amp: u64,
) -> StdResult<SimulationResponse> {
    let offer_amount = Decimal256::with_precision(offer_amount.to_uint256(), offer_asset_prec)?;

    let asset_amounts = asset_amounts
        .iter()
        .enumerate()
        .map(|(i, amount)| {
            if i == ask_ind {
                Decimal256::with_precision(amount.to_uint256(), ask_asset_prec)
            } else {
                Decimal256::with_precision(amount.to_uint256(), offer_asset_prec)
            }
        })
        .collect::<Result<Vec<Decimal256>, StdError>>()?;

    let total_offer_amount: Decimal256;
    let total_ask_amount: Decimal256;
    if ask_ind == 0 {
        total_offer_amount = asset_amounts[1];
        total_ask_amount = asset_amounts[0];
    } else {
        total_offer_amount = asset_amounts[0];
        total_ask_amount = asset_amounts[1];
    }

    if check_swap_parameters(asset_amounts.to_vec(), offer_amount).is_err() {
        return Ok(SimulationResponse {
            return_amount: Uint128::zero(),
            spread_amount: Uint128::zero(),
            commission_amount: Uint128::zero(),
        });
    }

    let SwapResult {
        return_amount,
        spread_amount,
    } = compute_swap(
        total_offer_amount,
        total_ask_amount,
        offer_amount,
        ask_asset_prec,
        &asset_amounts,
        block_time,
        init_amp_time,
        init_amp,
        next_amp_time,
        next_amp,
    )
    .map_err(|err| StdError::generic_err(format!("{err}")))?;

    let commission_amount = total_fee_rate.checked_mul_uint128(return_amount)?;
    let return_amount = return_amount.saturating_sub(commission_amount);

    Ok(SimulationResponse {
        return_amount,
        spread_amount,
        commission_amount,
    })
}

/// Checks swap parameters.
///
/// * **pools** amount of tokens in pools.
///
/// * **swap_amount** amount to swap.
pub fn check_swap_parameters(
    asset_amounts: Vec<Decimal256>,
    swap_amount: Decimal256,
) -> StdResult<()> {
    if asset_amounts.iter().any(|amount| amount.is_zero()) {
        return Err(StdError::generic_err("One of the assets is empty"));
    }

    if swap_amount.is_zero() {
        return Err(StdError::generic_err("Swap amount must not be zero"));
    }

    Ok(())
}

/// Structure for internal use which represents swap result.
struct SwapResult {
    pub return_amount: Uint128,
    pub spread_amount: Uint128,
}

fn compute_swap(
    total_offer_amount: Decimal256,
    total_ask_amount: Decimal256,
    offer_amount: Decimal256,
    ask_asset_prec: u8,
    asset_amounts: &[Decimal256],
    block_time: u64,
    init_amp_time: u64,
    init_amp: u64,
    next_amp_time: u64,
    next_amp: u64,
) -> Result<SwapResult, ContractError> {
    let new_total_ask_amount = calc_y(
        compute_current_amp(block_time, init_amp_time, init_amp, next_amp_time, next_amp)?,
        total_offer_amount + offer_amount,
        asset_amounts,
        ask_asset_prec,
    )?;

    let return_amount =
        total_ask_amount.to_uint128_with_precision(ask_asset_prec)? - new_total_ask_amount;
    let offer_amount = offer_amount.to_uint128_with_precision(ask_asset_prec)?;

    // We consider swap rate 1:1 in stable swap thus any difference is considered as spread.
    let spread_amount = offer_amount.saturating_sub(return_amount);

    Ok(SwapResult {
        return_amount,
        spread_amount,
    })
}
