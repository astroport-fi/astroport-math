use cosmwasm_std::{Decimal, Decimal256, Uint128};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

mod astroport;
mod utils;

#[wasm_bindgen]
pub fn concentrated_swap(
    offer_amount: &str,
    offer_asset_prec: &str,
    ask_ind: &str,
    ask_asset_prec: &str,
    asset_amounts: &str,
    maker_fee_share: &str,
    oracle_price: &str,
    price_scale: &str,
    fee_gamma: &str,
    mid_fee: &str,
    out_fee: &str,
    block_time: &str,
    initial_time: &str,
    inital_amp: &str,
    initial_gamma: &str,
    future_time: &str,
    future_amp: &str,
    future_gamma: &str,
) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let offer_amount = Decimal256::from_str(offer_amount)
        .map_err(|e| JsValue::from_str(&format!("Invalid offer_amount: {}", e)))?;

    let offer_asset_prec = offer_asset_prec
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid offer_asset_prec: {}", e)))?;

    let ask_ind = ask_ind
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid ask_ind: {}", e)))?;

    let ask_asset_prec = ask_asset_prec
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid ask_asset_prec: {}", e)))?;

    let asset_amounts = serde_json::from_str::<Vec<Decimal256>>(asset_amounts)
        .map_err(|e| JsValue::from_str(&format!("Invalid asset_amounts: {}", e)))?;

    let maker_fee_share = Decimal256::from_str(maker_fee_share)
        .map_err(|e| JsValue::from_str(&format!("Invalid maker_fee_share: {}", e)))?;

    let oracle_price = Decimal256::from_str(oracle_price)
        .map_err(|e| JsValue::from_str(&format!("Invalid oracle_price: {}", e)))?;

    let price_scale = Decimal256::from_str(price_scale)
        .map_err(|e| JsValue::from_str(&format!("Invalid price_scale: {}", e)))?;

    let fee_gamma = Decimal256::from_str(fee_gamma)
        .map_err(|e| JsValue::from_str(&format!("Invalid fee_gamma: {}", e)))?;

    let mid_fee = Decimal256::from_str(mid_fee)
        .map_err(|e| JsValue::from_str(&format!("Invalid mid_fee: {}", e)))?;

    let out_fee = Decimal256::from_str(out_fee)
        .map_err(|e| JsValue::from_str(&format!("Invalid out_fee: {}", e)))?;

    let block_time = block_time
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid block_time: {}", e)))?;

    let initial_time = initial_time
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid initial_time: {}", e)))?;

    let inital_amp = Decimal::from_str(inital_amp)
        .map_err(|e| JsValue::from_str(&format!("Invalid inital_amp: {}", e)))?;

    let initial_gamma = Decimal::from_str(initial_gamma)
        .map_err(|e| JsValue::from_str(&format!("Invalid initial_gamma: {}", e)))?;

    let future_time = future_time
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid future_time: {}", e)))?;

    let future_amp = Decimal::from_str(future_amp)
        .map_err(|e| JsValue::from_str(&format!("Invalid future_amp: {}", e)))?;

    let future_gamma = Decimal::from_str(future_gamma)
        .map_err(|e| JsValue::from_str(&format!("Invalid future_gamma: {}", e)))?;

    let result = astroport::pair_concentrated::swap::simulate(
        offer_amount,
        offer_asset_prec,
        ask_ind,
        ask_asset_prec,
        &asset_amounts,
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
    )
    .map_err(|e| JsValue::from_str(&format!("Error while simulating swap: {}", e)))?;

    let json_result = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Error while serializing result: {}", e)))?;

    Ok(JsValue::from_str(&json_result))
}

#[wasm_bindgen]
pub fn concentrated_provide() -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let result = astroport::pair_concentrated::provide::simulate()
        .map_err(|e| JsValue::from_str(&format!("Error while simulating provide: {}", e)))?;

    let json_result = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Error while serializing result: {}", e)))?;

    Ok(JsValue::from_str(&json_result))
}

#[wasm_bindgen]
pub fn concentrated_withdraw() -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let result = astroport::pair_concentrated::withdraw::simulate()
        .map_err(|e| JsValue::from_str(&format!("Error while simulating withdraw: {}", e)))?;

    let json_result = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Error while serializing result: {}", e)))?;

    Ok(JsValue::from_str(&json_result))
}

#[wasm_bindgen]
pub fn stable_swap(
    offer_amount: &str,
    offer_asset_prec: &str,
    ask_ind: &str,
    ask_asset_prec: &str,
    asset_amounts: &str,
    total_fee_rate: &str,
    block_time: &str,
    init_amp_time: &str,
    init_amp: &str,
    next_amp_time: &str,
    next_amp: &str,
) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let offer_amount = Decimal256::from_str(offer_amount)
        .map_err(|e| JsValue::from_str(&format!("Invalid offer_asset_prec: {}", e)))?;

    let offer_asset_prec = offer_asset_prec
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid offer_asset_prec: {}", e)))?;

    let ask_ind = ask_ind
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid ask_ind: {}", e)))?;

    let ask_asset_prec = ask_asset_prec
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid ask_asset_prec: {}", e)))?;

    let asset_amounts = serde_json::from_str::<Vec<Decimal256>>(asset_amounts)
        .map_err(|e| JsValue::from_str(&format!("Invalid asset_amounts: {}", e)))?;

    let total_fee_rate = total_fee_rate
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid total_fee_rate: {}", e)))?;

    let block_time = block_time
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid block_time: {}", e)))?;

    let init_amp_time = init_amp_time
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid init_amp_time: {}", e)))?;

    let init_amp = init_amp
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid init_amp: {}", e)))?;

    let next_amp_time = next_amp_time
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid next_amp_time: {}", e)))?;

    let next_amp = next_amp
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid next_amp: {}", e)))?;

    let result = astroport::pair_stable::swap::simulate(
        offer_amount,
        offer_asset_prec,
        ask_ind,
        ask_asset_prec,
        &asset_amounts,
        total_fee_rate,
        block_time,
        init_amp_time,
        init_amp,
        next_amp_time,
        next_amp,
    )
    .map_err(|e| JsValue::from_str(&format!("Error while simulating swap: {}", e)))?;

    let json_result = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Error while serializing result: {}", e)))?;

    Ok(JsValue::from_str(&json_result))
}

#[wasm_bindgen]
pub fn stable_provide(
    deposits: &str,
    asset_amounts: &str,
    asset_precisions: &str,
    total_share: &str,
    block_time: &str,
    init_amp_time: &str,
    init_amp: &str,
    next_amp_time: &str,
    next_amp: &str,
) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let deposits = serde_json::from_str::<Vec<Decimal256>>(deposits)
        .map_err(|e| JsValue::from_str(&format!("Invalid deposits: {}", e)))?;

    let asset_amounts = serde_json::from_str::<Vec<Decimal256>>(asset_amounts)
        .map_err(|e| JsValue::from_str(&format!("Invalid asset_amounts: {}", e)))?;

    let asset_precisions = serde_json::from_str::<Vec<u8>>(asset_precisions)
        .map_err(|e| JsValue::from_str(&format!("Invalid asset_precisions: {}", e)))?;

    let total_share = total_share
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid total_share: {}", e)))?;

    let block_time = block_time
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid block_time: {}", e)))?;

    let init_amp_time = init_amp_time
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid init_amp_time: {}", e)))?;

    let init_amp = init_amp
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid init_amp: {}", e)))?;

    let next_amp_time = next_amp_time
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid next_amp_time: {}", e)))?;

    let next_amp = next_amp
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid next_amp: {}", e)))?;

    let result = astroport::pair_stable::provide::simulate(
        &deposits,
        &asset_amounts,
        &asset_precisions,
        total_share,
        block_time,
        init_amp_time,
        init_amp,
        next_amp_time,
        next_amp,
    )
    .map_err(|e| JsValue::from_str(&format!("Error while simulating provide: {}", e)))?;

    let json_result = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Error while serializing result: {}", e)))?;

    Ok(JsValue::from_str(&json_result))
}

#[wasm_bindgen]
pub fn stable_withdraw(
    amount: &str,
    asset_amounts: &str,
    total_share: &str,
) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let amount = amount
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid amount: {}", e)))?;

    let asset_amounts = serde_json::from_str::<Vec<Uint128>>(asset_amounts)
        .map_err(|e| JsValue::from_str(&format!("Invalid asset_amounts: {}", e)))?;

    let total_share = total_share
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid total_share: {}", e)))?;

    let result = astroport::pair_stable::withdraw::simulate(amount, &asset_amounts, total_share)
        .map_err(|e| JsValue::from_str(&format!("Error while simulating withdraw: {}", e)))?;

    let json_result = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Error while serializing result: {}", e)))?;

    Ok(JsValue::from_str(&json_result))
}

#[wasm_bindgen]
pub fn xyk_swap(
    offer_amount: &str,
    ask_ind: &str,
    asset_amounts: &str,
    total_fee_rate: &str,
) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let offer_amount = offer_amount
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid offer_amount: {}", e)))?;

    let ask_ind = ask_ind
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid ask_ind: {}", e)))?;

    let asset_amounts = serde_json::from_str::<Vec<Uint128>>(asset_amounts)
        .map_err(|e| JsValue::from_str(&format!("Invalid asset_amounts: {}", e)))?;

    let total_fee_rate = total_fee_rate
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid total_fee_rate: {}", e)))?;

    let result =
        astroport::pair_xyk::swap::simulate(offer_amount, ask_ind, &asset_amounts, total_fee_rate)
            .map_err(|e| JsValue::from_str(&format!("Error while simulating swap: {}", e)))?;

    let json_result = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Error while serializing result: {}", e)))?;

    Ok(JsValue::from_str(&json_result))
}

#[wasm_bindgen]
pub fn xyk_provide(
    deposits: &str,
    asset_amounts: &str,
    total_share: &str,
) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let deposits = serde_json::from_str::<Vec<Uint128>>(deposits)
        .map_err(|e| JsValue::from_str(&format!("Invalid deposits: {}", e)))?;

    let asset_amounts = serde_json::from_str::<Vec<Uint128>>(asset_amounts)
        .map_err(|e| JsValue::from_str(&format!("Invalid asset_amounts: {}", e)))?;

    let total_share = total_share
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid total_share: {}", e)))?;

    let result = astroport::pair_xyk::provide::simulate(&deposits, &asset_amounts, total_share)
        .map_err(|e| JsValue::from_str(&format!("Error while simulating provide: {}", e)))?;

    let json_result = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Error while serializing result: {}", e)))?;

    Ok(JsValue::from_str(&json_result))
}

#[wasm_bindgen]
pub fn xyk_withdraw(
    amount: &str,
    asset_amounts: &str,
    total_share: &str,
) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let amount = amount
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid amount: {}", e)))?;

    let asset_amounts = serde_json::from_str::<Vec<Uint128>>(asset_amounts)
        .map_err(|e| JsValue::from_str(&format!("Invalid asset_amounts: {}", e)))?;

    let total_share = total_share
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid total_share: {}", e)))?;

    let result = astroport::pair_xyk::withdraw::simulate(amount, &asset_amounts, total_share)
        .map_err(|e| JsValue::from_str(&format!("Error while simulating withdraw: {}", e)))?;

    let json_result = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Error while serializing result: {}", e)))?;

    Ok(JsValue::from_str(&json_result))
}
