pub mod greeks;
pub mod test;

use crate::{decimals::DollarUSD, stock::Stock};
use chrono::{DateTime, Utc};
use core::fmt;
use decimal_rs::Decimal;
use std::{fmt::Display, str::FromStr};

use self::greeks::Greeks;

pub type Strike = DollarUSD;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum OptionType {
    Call,
    Put,
}

pub struct OptionCandidate {
    underlying: Box<Stock>,
    strike: Strike,
    expiration: DateTime<Utc>,
    option_type: OptionType,
}

impl OptionCandidate {
    pub fn new(
        underlying: Box<Stock>,
        strike: Strike,
        expiration: DateTime<Utc>,
        option_type: OptionType,
    ) -> OptionCandidate {
        OptionCandidate {
            underlying,
            strike,
            expiration,
            option_type,
        }
    }

    pub fn get_tte(&self) -> Decimal {
        let mte = self
            .expiration
            .signed_duration_since(Utc::now())
            .num_minutes();

        return Decimal::from(mte) * 1.9013E-6;
    }

    pub fn get_all_greeks(&self) -> Greeks {
        let s = self.underlying.get_price().get_decimal();
        let k = self.strike.get_decimal();
        let t = self.get_tte();
        let r = Decimal::from_str("0.02").unwrap();
        let v = self.underlying.get_hv();
        // println!("s: {:?}, k: {:?}, t: {}, r: {}, v: {}", s, k, t, r, v);
        let greeks = greeks::get_all_greeks(self.option_type, s, k, t, r, v);
        return greeks;
    }
}

impl Display for OptionCandidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OptionCandidate: {:?} {:?} {:?} {:?}",
            self.underlying, self.strike, self.expiration, self.option_type
        )
    }
}
