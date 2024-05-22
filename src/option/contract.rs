use crate::option::greeks;
use crate::option::OptionType;
use crate::option::Strike;
use crate::{decimals::DollarUSD, stock::Stock};
use chrono::{DateTime, Utc};
use core::fmt;
use rust_decimal::{Decimal, MathematicalOps};
use std::fmt::Display;

#[derive(PartialEq, Clone, Debug)]

pub struct Contract {
    underlying: Box<Stock>,
    strike: Strike,
    expiration: DateTime<Utc>,
    option_type: OptionType,
}

impl Contract {
    pub fn new(
        underlying: Box<Stock>,
        strike: Strike,
        expiration: DateTime<Utc>,
        option_type: OptionType,
    ) -> Contract {
        Contract {
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

        return Decimal::from(mte)
            * Decimal::from_f64_retain(1.9013E-6)
                .unwrap()
                .round_sf(4)
                .unwrap();
    }

    pub fn get_all_greeks(&self, rf_rate: Decimal) -> greeks::Greeks {
        let s = self.underlying.get_price().get_decimal();
        let k = self.strike.get_decimal();
        let t = self.get_tte();
        let v = self.underlying.get_hv();
        let d = self.underlying.get_dividend_yield();
        println!(
            "s: {}, k: {}, t: {}, r: {}, v: {}, d: {}",
            s, k, t, rf_rate, v, d
        );

        let greeks = greeks::get_all_greeks(self.option_type, s, k, t, rf_rate, v, d);
        return greeks;
    }

    pub fn get_iv() {
        todo!();
    }

    pub fn get_val_est(&self, rf_rate: Decimal) -> DollarUSD {
        let k = self.strike.get_decimal();
        let t = self.get_tte();
        let s = self.underlying.get_price().get_decimal();
        let v = self.underlying.get_hv();
        let d = self.underlying.get_dividend_yield().get_decimal();
        let d1: Decimal =
            ((s / k).ln() + (rf_rate + (v * v) / Decimal::from(2)) * t) / (v * t.sqrt().unwrap());
        let d2 = d1 - v * t.sqrt().unwrap();
        let nd1 = d1.norm_cdf();
        let nd2 = d2.norm_cdf();

        let rel_d = Decimal::exp(&(-(&d / s) * t));
        // let rel_r = Decimal::exp(&(-r * t));

        match self.option_type {
            OptionType::Call => {
                let val = s * nd1 - k * rel_d * nd2;
                return DollarUSD::new(val);
            }
            OptionType::Put => {
                let val = k * rel_d * (Decimal::ONE - nd2) - s * (Decimal::ONE - nd1);
                return DollarUSD::new(val);
            }
        }
    }
}

impl Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Contract: {:?} {:?} {:?} {:?}",
            self.underlying, self.strike, self.expiration, self.option_type
        )
    }
}
