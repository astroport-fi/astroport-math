use super::signed_decimal::SignedDecimal256;
use crate::astroport::pair_concentrated::consts::{MAX_ITER, N, N_POW2, TOL};
use cosmwasm_std::{Decimal256, StdError, StdResult};
use itertools::Itertools;

/// Internal constant to increase calculation accuracy.
const PADDING: Decimal256 = Decimal256::raw(1e36 as u128);

pub fn geometric_mean(x: &[Decimal256]) -> Decimal256 {
    (x[0] * x[1]).sqrt()
}

pub(crate) fn f(
    d: SignedDecimal256,
    x: &[SignedDecimal256],
    a: Decimal256,
    gamma: Decimal256,
) -> SignedDecimal256 {
    let mul = x[0] * x[1];
    let d_pow2 = d.pow(2);

    let k0 = mul * N_POW2 / d_pow2;
    let k = a * gamma.pow(2) * k0 / (SignedDecimal256::from(gamma + Decimal256::one()) - k0).pow(2);

    k * d * (x[0] + x[1]) + mul - k * d_pow2 - d_pow2 / N_POW2
}

/// df/dD
pub(crate) fn df_dd(
    d: SignedDecimal256,
    x: &[SignedDecimal256],
    a: Decimal256,
    gamma: Decimal256,
) -> SignedDecimal256 {
    let mul = x[0] * x[1];
    let a_gamma_pow_2 = a * gamma.pow(2); // A * gamma^2

    let k0 = mul * N_POW2 / d.pow(2);

    let gamma_one_k0 = SignedDecimal256::from(gamma + Decimal256::one()) - k0; // gamma + 1 - K0
    let gamma_one_k0_pow2 = gamma_one_k0.pow(2); // (gamma + 1 - K0)^2

    let k = a_gamma_pow_2 * k0 / gamma_one_k0_pow2;

    let k_d_denom = PADDING * d.pow(3) * gamma_one_k0_pow2 * gamma_one_k0;
    let k_d = -mul * N.pow(3) * a_gamma_pow_2 * (gamma + Decimal256::one() + k0);

    (k_d * d * PADDING / k_d_denom + k) * (x[0] + x[1])
        - (k_d * d * PADDING / k_d_denom + N * k) * d
        - (d / N)
}

pub(crate) fn newton_d(
    x: &[Decimal256],
    a: Decimal256,
    gamma: Decimal256,
) -> StdResult<Decimal256> {
    let mut d_prev: SignedDecimal256 = (N * geometric_mean(x)).into();
    let x = x.iter().map(SignedDecimal256::from).collect_vec();

    for _ in 0..MAX_ITER {
        let d = d_prev - f(d_prev, &x, a, gamma) / df_dd(d_prev, &x, a, gamma);
        if d.diff(d_prev) <= TOL {
            return d.try_into();
        }
        d_prev = d;
    }

    Err(StdError::generic_err("newton_d is not converging"))
}

/// df/dx
pub(crate) fn df_dx(
    d: Decimal256,
    x: &[SignedDecimal256],
    a: Decimal256,
    gamma: Decimal256,
    i: usize,
) -> SignedDecimal256 {
    let x_r = x[1 - i];
    let d_pow2 = d.pow(2);

    let k0 = x[0] * x[1] * N_POW2 / d_pow2;
    let gamma_one_k0 = gamma + Decimal256::one() - k0;
    let gamma_one_k0_pow2 = gamma_one_k0.pow(2);
    let a_gamma_pow2 = a * gamma.pow(2);

    let k = a_gamma_pow2 * k0 / gamma_one_k0_pow2;
    let k0_x = x_r * N_POW2;
    let k_x = k0_x * a_gamma_pow2 * (gamma + Decimal256::one() + k0) * PADDING
        / (PADDING * d_pow2 * gamma_one_k0 * gamma_one_k0_pow2);

    (k_x * (x[0] + x[1]) + k) * d + x_r - k_x * d_pow2
}

pub(crate) fn newton_y(
    xs: &[Decimal256],
    a: Decimal256,
    gamma: Decimal256,
    d: Decimal256,
    j: usize,
) -> StdResult<Decimal256> {
    let mut x = xs.iter().map(SignedDecimal256::from).collect_vec();
    let x0 = d.pow(2) / (N_POW2 * x[1 - j]);
    let mut xi_1 = x0;
    x[j] = x0;

    for _ in 0..MAX_ITER {
        let xi = xi_1 - f(d.into(), &x, a, gamma) / df_dx(d, &x, a, gamma, j);
        if xi.diff(xi_1) <= TOL {
            return xi.try_into();
        }
        x[j] = xi;
        xi_1 = xi;
    }

    Err(StdError::generic_err("newton_y is not converging"))
}
