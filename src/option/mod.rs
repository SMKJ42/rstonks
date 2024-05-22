mod contract;
mod greeks;
pub mod test;

use crate::decimals::DollarUSD;
pub type Strike = DollarUSD;
pub use contract::Contract;
pub use greeks::Greeks;

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum OptionType {
    Call,
    Put,
}
