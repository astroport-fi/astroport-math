use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, StdResult, Uint128};

#[cw_serde]
pub struct SimulationResponse {
    /// The amount of assets returned by the withdraw
    pub returned_amounts: Vec<Uint128>,
}

pub fn simulate(
    amount: Uint128,
    asset_amounts: &[Uint128],
    total_share: Uint128,
) -> StdResult<SimulationResponse> {
    let returned_amounts = compute_withdraw(amount, asset_amounts, total_share);

    Ok(SimulationResponse { returned_amounts })
}

pub fn compute_withdraw(
    amount: Uint128,
    asset_amounts: &[Uint128],
    total_share: Uint128,
) -> Vec<Uint128> {
    let mut share_ratio = Decimal::zero();
    if !total_share.is_zero() {
        share_ratio = Decimal::from_ratio(amount, total_share);
    }

    let mut refund_assets = Vec::with_capacity(asset_amounts.len());

    for asset_amount in asset_amounts.iter() {
        refund_assets.push(asset_amount.clone() * share_ratio);
    }

    refund_assets
}
