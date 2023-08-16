use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Decimal256};

use crate::astroport::cosmwasm_ext::IntegerToDecimal;

use super::consts::{FEE_TOL, N_POW2};

#[cw_serde]
#[derive(Default, Copy)]
pub struct AmpGamma {
    pub amp: Decimal,
    pub gamma: Decimal,
}

pub fn get_amp_gamma(
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

pub fn fee(
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
