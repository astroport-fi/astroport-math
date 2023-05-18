use self::{
    consts::{FEE_TOL, N_POW2},
    error::ContractError,
    math::{calc_d, calc_y},
    state::AmpGamma,
};
use super::cosmwasm_ext::{Decimal256Ext, DecimalToInteger};
use crate::astroport::cosmwasm_ext::IntegerToDecimal;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Decimal256, Fraction, StdError, StdResult, Uint128};

pub mod consts;
pub mod error;
pub mod math;
pub mod state;

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
    offer_asset_prec: u32,
    ask_ind: usize,
    ask_asset_prec: u32,
    asset_amounts: &[Decimal256],
    maker_fee_share: Decimal256,
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
) -> Result<SimulationResponse, ContractError> {
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

    Ok(SimulationResponse {
        return_amount: swap_result.dy.to_uint(ask_asset_prec)?,
        spread_amount: swap_result.spread_fee.to_uint(ask_asset_prec)?,
        commission_amount: swap_result.total_fee.to_uint(ask_asset_prec)?,
    })
}

#[cw_serde]
pub struct SwapResult {
    pub new_y: Decimal256,
    pub price: Decimal256,
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

    let offer_amount = if offer_ind == 1 {
        offer_amount * price_scale
    } else {
        offer_amount
    };

    ixs[offer_ind] += offer_amount;

    let new_y = calc_y(&ixs, d, &amp_gamma, ask_ind)?;
    let mut dy = ixs[ask_ind] - new_y;
    ixs[ask_ind] = new_y;

    let price = if ask_ind == 1 {
        dy /= price_scale;
        price_scale.inv().unwrap()
    } else {
        price_scale
    };

    // Since price_scale moves slower than real price spread fee may become negative
    let spread_fee = (offer_amount * price).saturating_sub(dy);

    let fee_rate = fee(&ixs, fee_gamma, mid_fee, out_fee);
    let total_fee = fee_rate * dy;
    dy -= total_fee;

    Ok(SwapResult {
        new_y,
        price,
        offer_amount,
        dy,
        spread_fee,
        maker_fee: total_fee * maker_fee_share,
        total_fee,
    })
}

fn get_amp_gamma(
    block_time: u64,
    initial_time: u64,
    inital_amp: Decimal,
    initial_gamma: Decimal,
    future_time: u64,
    future_amp: Decimal,
    future_gamma: Decimal,
) -> AmpGamma {
    if block_time < future_time {
        let total = (future_time - initial_time).to_decimal();
        let passed = (block_time - initial_time).to_decimal();
        let left = total - passed;

        // A1 = A0 + (A1 - A0) * (block_time - t_init) / (t_end - t_init) -> simplified to:
        // A1 = ( A0 * (t_end - block_time) + A1 * (block_time - t_init) ) / (t_end - t_init)
        let amp = (inital_amp * left + future_amp * passed) / total;
        let gamma = (initial_gamma * left + future_gamma * passed) / total;

        AmpGamma { amp, gamma }
    } else {
        AmpGamma {
            amp: future_amp,
            gamma: future_gamma,
        }
    }
}

fn fee(
    xp: &[Decimal256],
    fee_gamma: Decimal256,
    mid_fee: Decimal256,
    out_fee: Decimal256,
) -> Decimal256 {
    let sum = xp[0] + xp[1];
    let mut k = xp[0] * xp[1] * N_POW2 / sum.pow(2);
    k = fee_gamma / (fee_gamma + Decimal256::one() - k);

    if k <= FEE_TOL {
        k = Decimal256::zero()
    }

    k * mid_fee + (Decimal256::one() - k) * out_fee
}
