use super::{
    error::ContractError,
    math::{calc_d, calc_y},
    state::{fee, get_amp_gamma},
};
use crate::astroport::cosmwasm_ext::{Decimal256Ext, DecimalToInteger};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Decimal256, StdError, StdResult, Uint128};

#[cw_serde]
pub struct SwapSimulationResponse {
    /// The amount of ask assets returned by the swap
    pub return_amount: Uint128,
    /// The spread used in the swap operation
    pub spread_amount: Uint128,
    /// The amount of fees charged by the transaction
    pub commission_amount: Uint128,
}

pub fn simulate(
    offer_amount: Decimal256,
    offer_asset_prec: u32,
    ask_ind: usize,
    ask_asset_prec: u32,
    asset_amounts: &[Decimal256],
    maker_fee_share: Decimal256,
    oracle_price: Decimal256,
    price_scale: Decimal256,
    fee_gamma: Decimal256,
    mid_fee: Decimal256,
    out_fee: Decimal256,
    block_time: u64,
    initial_time: u64,
    inital_amp: Decimal,
    initial_gamma: Decimal,
    future_time: u64,
    future_amp: Decimal,
    future_gamma: Decimal,
) -> Result<SwapSimulationResponse, ContractError> {
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

    let swap_result = compute_swap(
        &asset_amounts,
        offer_amount,
        ask_ind,
        maker_fee_share,
        oracle_price,
        price_scale,
        fee_gamma,
        mid_fee,
        out_fee,
        block_time,
        initial_time,
        inital_amp,
        initial_gamma,
        future_time,
        future_amp,
        future_gamma,
    )?;

    Ok(SwapSimulationResponse {
        return_amount: swap_result.dy.to_uint(ask_asset_prec)?,
        spread_amount: swap_result.spread_fee.to_uint(ask_asset_prec)?,
        commission_amount: swap_result.total_fee.to_uint(ask_asset_prec)?,
    })
}

#[cw_serde]
pub struct SwapResult {
    pub new_y: Decimal256,
    pub offer_amount: Decimal256,
    pub dy: Decimal256,
    pub spread_fee: Decimal256,
    pub maker_fee: Decimal256,
    pub total_fee: Decimal256,
}

fn compute_swap(
    xs: &[Decimal256],
    offer_amount: Decimal256,
    ask_ind: usize,
    maker_fee_share: Decimal256,
    oracle_price: Decimal256,
    price_scale: Decimal256,
    fee_gamma: Decimal256,
    mid_fee: Decimal256,
    out_fee: Decimal256,
    block_time: u64,
    initial_time: u64,
    inital_amp: Decimal,
    initial_gamma: Decimal,
    future_time: u64,
    future_amp: Decimal,
    future_gamma: Decimal,
) -> StdResult<SwapResult> {
    let offer_ind = 1 ^ ask_ind;

    let mut ixs = xs.to_vec();
    ixs[1] *= price_scale;

    let amp_gamma = get_amp_gamma(
        block_time,
        initial_time,
        inital_amp,
        initial_gamma,
        future_time,
        future_amp,
        future_gamma,
    );
    let d = calc_d(&ixs, &amp_gamma)?;

    if offer_ind == 1 {
        ixs[offer_ind] += offer_amount * price_scale;
    } else {
        ixs[offer_ind] += offer_amount;
    }

    let new_y = calc_y(&ixs, d, &amp_gamma, ask_ind)?;
    let mut dy = ixs[ask_ind] - new_y;
    ixs[ask_ind] = new_y;

    let spread_fee = if ask_ind == 1 {
        dy /= price_scale;
        (offer_amount / oracle_price).saturating_sub(dy)
    } else {
        offer_amount.saturating_sub(dy / oracle_price)
    };

    let fee_rate = fee(&ixs, fee_gamma, mid_fee, out_fee);
    let total_fee = fee_rate * dy;
    dy -= total_fee;

    Ok(SwapResult {
        new_y,
        offer_amount,
        dy,
        spread_fee,
        maker_fee: total_fee * maker_fee_share,
        total_fee,
    })
}
