use super::math::math_decimal::{newton_d, newton_y};
use super::state::AmpGamma;
use cosmwasm_std::{Decimal256, StdResult};

mod math_decimal;
mod signed_decimal;

/// Calculate D invariant based on known pool volumes.
///
/// * **xs** - internal representation of pool volumes.
/// * **amp_gamma** - an object which represents current Amp and Gamma parameters.
pub fn calc_d(xs: &[Decimal256], amp_gamma: &AmpGamma) -> StdResult<Decimal256> {
    newton_d(xs, amp_gamma.amp.into(), amp_gamma.gamma.into())
}

/// Calculate unknown pool's volume based on the other side of pools which is known and D.
///
/// * **xs** - internal representation of pool volumes.
/// * **d** - current D invariant.
/// * **amp_gamma** - an object which represents current Amp and Gamma parameters.
/// * **ask_ind** - the index of pool which is unknown.
pub fn calc_y(
    xs: &[Decimal256],
    d: Decimal256,
    amp_gamma: &AmpGamma,
    ask_ind: usize,
) -> StdResult<Decimal256> {
    newton_y(xs, amp_gamma.amp.into(), amp_gamma.gamma.into(), d, ask_ind)
}
