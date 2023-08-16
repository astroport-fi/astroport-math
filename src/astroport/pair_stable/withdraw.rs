use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdResult, Uint128};

use crate::astroport::pair_xyk::withdraw::compute_withdraw;

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
