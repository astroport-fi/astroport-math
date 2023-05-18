/* tslint:disable */
/* eslint-disable */
/**
* @param {string} offer_amount
* @param {string} offer_asset_prec
* @param {string} ask_ind
* @param {string} ask_asset_prec
* @param {string} asset_amounts
* @param {string} maker_fee_share
* @param {string} price_scale
* @param {string} fee_gamma
* @param {string} mid_fee
* @param {string} out_fee
* @param {string} block_time
* @param {string} initial_time
* @param {string} inital_amp
* @param {string} initial_gamma
* @param {string} future_time
* @param {string} future_amp
* @param {string} future_gamma
* @returns {any}
*/
export function concentrated_swap(offer_amount: string, offer_asset_prec: string, ask_ind: string, ask_asset_prec: string, asset_amounts: string, maker_fee_share: string, price_scale: string, fee_gamma: string, mid_fee: string, out_fee: string, block_time: string, initial_time: string, inital_amp: string, initial_gamma: string, future_time: string, future_amp: string, future_gamma: string): any;
/**
* @param {string} offer_amount
* @param {string} ask_ind
* @param {string} asset_amounts
* @param {string} total_fee_rate
* @returns {any}
*/
export function xyk_swap(offer_amount: string, ask_ind: string, asset_amounts: string, total_fee_rate: string): any;
/**
* @param {string} offer_amount
* @param {string} offer_asset_prec
* @param {string} ask_ind
* @param {string} ask_asset_prec
* @param {string} asset_amounts
* @param {string} total_fee_rate
* @param {string} block_time
* @param {string} init_amp_time
* @param {string} init_amp
* @param {string} next_amp_time
* @param {string} next_amp
* @returns {any}
*/
export function stable_swap(offer_amount: string, offer_asset_prec: string, ask_ind: string, ask_asset_prec: string, asset_amounts: string, total_fee_rate: string, block_time: string, init_amp_time: string, init_amp: string, next_amp_time: string, next_amp: string): any;
