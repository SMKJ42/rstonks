mod black_scholes;
mod contract;
mod test;

use std::fmt::Display;

use crate::decimals::DollarUSD;
pub type Strike = DollarUSD;
pub use black_scholes::BlackScholes;
pub use contract::OptionContract;

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum ContractType {
    Call,
    Put,
}

impl Display for ContractType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
