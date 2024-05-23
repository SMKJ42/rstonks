mod price_data;
mod test;

use self::price_data::{ClosePriceData, PriceData};
use crate::decimals::DollarUSD;
use crate::stock_option::{BlackScholes, ContractType, OptionContract, Strike};
use chrono::{DateTime, Utc};
use core::fmt;
use rust_decimal::Decimal;
use std::fmt::Display;
#[derive(Clone, Debug, PartialEq)]
pub struct Stock {
    dividend_yield: DollarUSD,
    dy_close_price_data: ClosePriceData,
    price_data: PriceData,
    price: DollarUSD,
    ticker: String,
    name: String,
}

impl Stock {
    pub fn new(
        name: String,
        ticker: String,
        price: DollarUSD,
        price_data: PriceData,
        close_price_data: ClosePriceData,
        dividend: DollarUSD,
    ) -> Stock {
        Stock {
            dividend_yield: dividend,
            dy_close_price_data: close_price_data,
            price_data,
            price,
            ticker,
            name,
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
        return self.price.get_dollars();
    }

    pub fn get_price_as_decimal(&self) -> Decimal {
        return self.price.get_decimal();
    }

    pub fn get_dividend_yield(&self) -> DollarUSD {
        return self.dividend_yield;
    }

    pub fn update_price(&mut self, price: DollarUSD) {
        self.price = price;
    }

    pub fn update_price_data(&mut self, data: PriceData) {
        self.price_data = data;
        todo!();
    }

    pub fn create_option_contract<'a>(
        &'a self,
        strike: Strike,
        price: DollarUSD,
        expiration: DateTime<Utc>,
        option_type: ContractType,
    ) -> OptionContract<'a> {
        return OptionContract::new(&self, price, strike, expiration, option_type);
    }

    pub fn get_greeks_for_option(option: OptionContract, rf_rate: Decimal) -> BlackScholes {
        let iv = option.get_iv(rf_rate);
        return option.get_all_greeks(rf_rate, iv);
    }
}

impl Display for Stock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {}, Ticker: {}, Price: {}",
            self.name, self.ticker, self.price
        )
    }
}
