use crate::decimals::DollarUSD;
use decimal_rs::Decimal;

pub type PriceData = Vec<Option<(DollarUSD, DollarUSD, DollarUSD, DollarUSD)>>;
pub type ClosePriceData = Vec<Option<DollarUSD>>;

pub fn get_historical_volatility(price_data: Vec<DollarUSD>) -> Decimal {
    let mut sum: DollarUSD = DollarUSD::new(Decimal::from(0));
    let mut count = 0;
    let mut prev_price = price_data[0];
    let mut log_returns: Vec<Decimal> = Vec::new();

    for i in 1..price_data.len() {
        let log_return = DollarUSD::ln(price_data[i] / prev_price);
        prev_price = price_data[i];
        log_returns.push(log_return.get_decimal());
        sum += log_return;
        count += 1;
    }

    let log_mean = sum.get_decimal() / count;

    let mut sum: Decimal = Decimal::from(0);

    for log in log_returns.clone() {
        let curr: Decimal = log - log_mean;
        sum += curr.checked_pow(&Decimal::from(2)).unwrap();
    }

    let standard_deviation = (sum / (log_returns.len() - 1)).sqrt().unwrap();

    return standard_deviation * count;
}
