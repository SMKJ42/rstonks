use crate::decimals::DollarUSD;
use crate::stock::Stock;
use crate::stock_option::{ContractType, Strike};
use chrono::{DateTime, Utc};
use core::fmt;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::fmt::Display;
use std::rc::Rc;

use super::BlackScholes;

#[derive(PartialEq, Clone, Debug)]

pub struct OptionContract<'a> {
    underlying: Rc<&'a Stock>,
    market_price: DollarUSD,
    strike: Strike,
    expiration: DateTime<Utc>,
    option_type: ContractType,
    iv: Option<Decimal>,
    d1: Option<Decimal>,
    d2: Option<Decimal>,
}

impl<'a> OptionContract<'a> {
    pub fn new(
        underlying: &Stock,
        market_price: DollarUSD,
        strike: Strike,
        expiration: DateTime<Utc>,
        option_type: ContractType,
    ) -> OptionContract {
        OptionContract {
            underlying: Rc::new(underlying),
            market_price,
            strike,
            expiration,
            option_type,
            iv: None,
            d1: None,
            d2: None,
        }
    }

    pub fn get_iv(&self, rf_rate: Decimal) -> Decimal {
        let tol = Decimal::from_f64(1e-4).unwrap();
        let mut low = Decimal::from_f64(1e-6).unwrap();
        let mut mid = self.underlying.get_hv();
        let mut high = Decimal::from(5);
        while (high - low).abs() > tol {
            let curr = self.get_price_est(rf_rate, mid);
            if curr > self.market_price {
                high = mid;
            } else {
                low = mid;
            }
            mid = (low + high) / Decimal::TWO;
        }
        return mid;
    }

    fn get_hv_price_est(&self, rf_rate: Decimal) -> DollarUSD {
        let k = self.strike.get_decimal();
        let t = self.get_tte();
        let s = self.underlying.get_price_as_decimal();
        let v = self.underlying.get_hv();
        let d = self.underlying.get_dividend_yield();
        let o_type = self.option_type;

        return BlackScholes::price_option(k, t, s, v, d, rf_rate, o_type);
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

    pub fn get_price_est(&self, rf_rate: Decimal, v: Decimal) -> DollarUSD {
        let k = self.strike.get_decimal();
        let t = self.get_tte();
        let s = self.underlying.get_price_as_decimal();
        let d = self.underlying.get_dividend_yield();
        let o_type = self.option_type;
        return BlackScholes::price_option(k, t, s, v, d, rf_rate, o_type);
    }

    pub fn get_market_price(&self) -> DollarUSD {
        return self.market_price;
    }

    pub fn get_all_greeks(&self, rf_rate: Decimal, v: Decimal) -> BlackScholes {
        let s = self.underlying.get_price_as_decimal();
        let k = self.strike.get_decimal();
        let t = self.get_tte();
        let d = self.underlying.get_dividend_yield();
        let o_type = self.option_type;
        return BlackScholes::greeks(s, k, t, v, d, rf_rate, o_type);
    }
}

impl Display for OptionContract<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OptionContract: \n{} {} {} {}",
            self.underlying,
            self.strike,
            self.expiration.to_rfc2822(),
            self.option_type
        )
    }
}
