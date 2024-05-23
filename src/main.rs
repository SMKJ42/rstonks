mod decimals;
mod stock;
mod stock_option;

use std::str::FromStr;

use chrono::Utc;
use decimals::DollarUSD;
use rust_decimal::Decimal;
use stock::Stock;
use stock_option::{ContractType, OptionContract};

fn main() {
    let now = Utc::now();
    let mut ticker = Stock::new(
        "Test".to_string(),
        "TST".to_string(),
        DollarUSD::parse("$796.34"),
        vec![],
        vec![],
        DollarUSD::parse("$4.64"),
    );

    ticker.set_cp_data(vec![
        Some(DollarUSD::parse("$750.0")),
        Some(DollarUSD::parse("$750.0")),
        Some(DollarUSD::parse("$750.0")),
        Some(DollarUSD::parse("$775.5")),
        Some(DollarUSD::parse("$700.0")),
        Some(DollarUSD::parse("$809.1")),
        Some(DollarUSD::parse("$800.0")),
        Some(DollarUSD::parse("$805.0")),
    ]);

    let five_days_from_now = now.checked_add_signed(chrono::Duration::days(120)).unwrap();
    let option = OptionContract::new(
        &ticker,
        DollarUSD::parse("$42.85"),
        DollarUSD::parse("$800.0"),
        five_days_from_now,
        ContractType::Call,
    );

    let rf_rate = Decimal::from_str("0.0443").unwrap();
    let hv = ticker.get_hv();
    let iv = option.get_iv(rf_rate);
    let greeks = option.get_all_greeks(rf_rate, iv);
    let price_est = option.get_price_est(rf_rate, iv);
    let price_with_hv = option.get_price_est(rf_rate, hv);
    println!("hv {}", hv);
    println!("price with hv: {}", price_with_hv);
    println!("iv: {}", iv);
    println!("price with iv: {}", price_est);
    println!("{}", greeks);
    println!("{}, {}", option, price_est);
}
