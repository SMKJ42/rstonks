use crate::decimals::DollarUSD;
use rust_decimal::{Decimal, MathematicalOps};

pub type PriceData = Vec<Option<(DollarUSD, DollarUSD, DollarUSD, DollarUSD)>>;
pub type ClosePriceData = Vec<Option<DollarUSD>>;

pub fn get_historical_volatility(price_data: Vec<DollarUSD>) -> Decimal {
    let mut log_sum = Decimal::from(0);
    let mut prev_price = price_data[0];
    let mut log_returns: Vec<Decimal> = Vec::new();
    let count = &price_data.len() - 1;

    for i in 1..price_data.len() {
        let log_return = Decimal::ln(&(price_data[i].get_decimal() / prev_price.get_decimal()));
        prev_price = price_data[i];
        log_returns.push(log_return);
        log_sum += log_return;
    }

    let log_mean = log_sum / Decimal::from(count);

    let mut pow_sum: Decimal = Decimal::from(0);

    for log in log_returns {
        let curr: Decimal = log - log_mean;
        pow_sum += curr.powu(2);
    }

    let standard_deviation = (pow_sum / Decimal::from(count - 1)).sqrt().unwrap();

    return standard_deviation * Decimal::from(count + 1).sqrt().unwrap();
}
