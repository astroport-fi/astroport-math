use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Decimal256, StdResult, Uint128, Uint256};

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
    offer_amount: Uint128,
    ask_ind: usize,
    asset_amounts: &[Uint128],
    total_fee_rate: Decimal,
) -> StdResult<SimulationResponse> {
    let total_offer_amount: Uint128;
    let total_ask_amount: Uint128;
    if ask_ind == 0 {
        total_offer_amount = asset_amounts[1];
        total_ask_amount = asset_amounts[0];
    } else {
        total_offer_amount = asset_amounts[0];
        total_ask_amount = asset_amounts[1];
    }

    let (return_amount, spread_amount, commission_amount) = compute_swap(
        total_offer_amount,
        total_ask_amount,
        offer_amount,
        total_fee_rate,
    )?;

    Ok(SimulationResponse {
        return_amount,
        spread_amount,
        commission_amount,
    })
}

pub fn compute_swap(
    offer_pool: Uint128,
    ask_pool: Uint128,
    offer_amount: Uint128,
    commission_rate: Decimal,
) -> StdResult<(Uint128, Uint128, Uint128)> {
    let offer_pool: Uint256 = offer_pool.into();
    let ask_pool: Uint256 = ask_pool.into();
    let offer_amount: Uint256 = offer_amount.into();
    let commission_rate = Decimal256::from(commission_rate);

    // ask_amount = (ask_pool - cp / (offer_pool + offer_amount))
    let cp: Uint256 = offer_pool * ask_pool;
    let return_amount: Uint256 = (Decimal256::from_ratio(ask_pool, 1u8)
        - Decimal256::from_ratio(cp, offer_pool + offer_amount))
        * Uint256::from(1u8);

    // Calculate spread & commission
    let spread_amount: Uint256 =
        (offer_amount * Decimal256::from_ratio(ask_pool, offer_pool)).saturating_sub(return_amount);
    let commission_amount: Uint256 = return_amount * commission_rate;

    // The commision (minus the part that goes to the Maker contract) will be absorbed by the pool
    let return_amount: Uint256 = return_amount - commission_amount;
    Ok((
        return_amount.try_into()?,
        spread_amount.try_into()?,
        commission_amount.try_into()?,
    ))
}
