use std::{ops::{Mul, Div}, convert::TryInto};

use cosmwasm_std::Uint128;


pub fn _get_asset_value(price: Uint128, amount: Uint128) -> Uint128 {
    return amount.mul(Uint128::from(price));
}

pub fn _is_valid_cdp(c_price: Uint128, d_price: Uint128, c_amount: Uint128, d_amount: Uint128, c_decimal: u64, mcr: u64)  -> bool {
    let (collateral_value_times100_point00000, d_value) = _calculate_values(c_price, d_price, c_amount, d_amount);
    let decimal: u32 = c_decimal.try_into().unwrap();
    let c_decimal128 = Uint128::from(u64::pow(10, decimal));
    let mid = Uint128::from(u64::pow(10, 9-decimal));
    let d_value_adjusted = d_value.div(c_decimal128);
    if d_value_adjusted == Uint128::zero()  {
        return true;
    } else {
        // Suppose stablecoin decimal is 9, then calculate collateral ratio
        let cr = collateral_value_times100_point00000.mul(mid).div(d_value);
        return cr >= Uint128::from(mcr);
    }
}

pub fn _cr(c_price: Uint128, d_price: Uint128, c_amount: Uint128, d_amount: Uint128, c_decimal: u64, mcr: u64) -> Uint128 {
    let (collateral_value_times100_point00000, d_value) = _calculate_values(c_price, d_price, c_amount, d_amount);
    let decimal: u32 = c_decimal.try_into().unwrap();
    let c_decimal128 = Uint128::from(u64::pow(10, decimal));
    let mid = Uint128::from(u64::pow(10, 9-decimal));
    let d_value_adjusted = d_value.div(c_decimal128);
    if d_value_adjusted == Uint128::zero()  {
        return Uint128::from(9000u64);
    } else {
        // Suppose stablecoin decimal is 9, then calculate collateral ratio
        let cr = collateral_value_times100_point00000.mul(mid).div(d_value);
        return cr;
    }

}

pub fn _calculate_values(c_price: Uint128, d_price: Uint128, c_amount: Uint128, d_amount: Uint128) -> (Uint128, Uint128) {
    let c_value = c_price * c_amount;
    let d_value = d_price * d_amount;
    let collateral_value_times100_point00000 = c_value * Uint128::from(10000000u64);
    return (collateral_value_times100_point00000, d_value); 
}
