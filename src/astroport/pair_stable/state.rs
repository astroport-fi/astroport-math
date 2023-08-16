use cosmwasm_std::{StdResult, Uint128, Uint64};

/// Compute the current pool amplification coefficient (AMP).
pub fn compute_current_amp(
    block_time: u64,
    init_amp_time: u64,
    init_amp: u64,
    next_amp_time: u64,
    next_amp: u64,
) -> StdResult<Uint64> {
    if block_time < next_amp_time {
        let elapsed_time: Uint128 = block_time.saturating_sub(init_amp_time).into();
        let time_range = next_amp_time.saturating_sub(init_amp_time).into();
        let init_amp = Uint128::from(init_amp);
        let next_amp = Uint128::from(next_amp);

        if next_amp > init_amp {
            let amp_range = next_amp - init_amp;
            let res = init_amp + (amp_range * elapsed_time).checked_div(time_range)?;
            Ok(res.try_into()?)
        } else {
            let amp_range = init_amp - next_amp;
            let res = init_amp - (amp_range * elapsed_time).checked_div(time_range)?;
            Ok(res.try_into()?)
        }
    } else {
        Ok(Uint64::from(next_amp))
    }
}

pub fn greatest_precision(precisions: &[u8]) -> u8 {
    precisions.iter().max().copied().unwrap_or(0)
}
