use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
#[derive(Default, Copy)]
pub struct AmpGamma {
    pub amp: Decimal,
    pub gamma: Decimal,
}
