use self::price_data::{ClosePriceData, PriceData};
use crate::{
    decimals::DollarUSD,
    option::{greeks::Greeks, OptionCandidate, OptionType, Strike},
};
use chrono::{DateTime, Utc};
use core::fmt;
use decimal_rs::Decimal;
use std::fmt::Display;

pub mod price_data;
mod test;

#[derive(Clone, Debug)]
pub struct Stock {
    dividend_yield: Option<f64>,
    dy_close_price_data: ClosePriceData,
    price_data: PriceData,
    price: DollarUSD,
    ticker: String,
    name: String,
}

impl Stock {
    pub fn new() -> Stock {
        Stock {
            dividend_yield: None,
            dy_close_price_data: ClosePriceData::new(),
            price_data: PriceData::new(),
            price: DollarUSD::new(Decimal::from(0)),
            ticker: String::new(),
            name: String::new(),
        }
    }

    pub fn set_cp_data(&mut self, data: ClosePriceData) {
        self.dy_close_price_data = data;
    }

    pub fn get_hv(&self) -> Decimal {
        let mut hp: Vec<DollarUSD> = Vec::new();
        for data in self.dy_close_price_data.iter() {
            if data.as_ref().is_some() {
                hp.push(data.unwrap());
            }
        }

        return price_data::get_historical_volatility(hp);
    }

    pub fn get_price(&self) -> DollarUSD {
        return self.price;
    }

    pub fn update_price(&mut self, price: DollarUSD) {
        self.price = price;
    }

    pub fn update_price_data(&mut self, data: PriceData) {
        self.price_data = data;
        todo!();
    }

    pub fn get_greeks_for_option(
        self,
        strike: Strike,
        expiration: DateTime<Utc>,
        option_type: OptionType,
    ) -> Greeks {
        let option = OptionCandidate::new(Box::new(self), strike, expiration, option_type);

        return option.get_all_greeks();
    }
}

impl Display for Stock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {}, Ticker: {}, Stock: {:?}",
            self.name, self.ticker, self.price
        )
    }
}