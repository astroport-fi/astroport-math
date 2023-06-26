use cosmwasm_std::Decimal256;

/// ## Internal constants
/// Number of coins. (2.0)
pub const N: Decimal256 = Decimal256::raw(2000000000000000000);
/// Defines fee tolerance. If k coefficient is small enough then k = 0. (0.001)
pub const FEE_TOL: Decimal256 = Decimal256::raw(1000000000000000);
/// N ^ 2
pub const N_POW2: Decimal256 = Decimal256::raw(4000000000000000000);
/// 1e-5
pub const TOL: Decimal256 = Decimal256::raw(10000000000000);
/// Iterations limit for Newton's method
pub const MAX_ITER: usize = 64;
