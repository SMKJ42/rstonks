mod decimals;
mod option;
mod stock;

use std::str::FromStr;

use chrono::Utc;
use decimals::DollarUSD;
use option::OptionType;
use rust_decimal::Decimal;
use stock::Stock;

use crate::option::Contract;

fn main() {
    let now = Utc::now();
    let mut ticker = Stock::new(
        "Test".to_string(),
        "TST".to_string(),
        DollarUSD::parse("$800.94"),
        vec![],
        vec![],
        DollarUSD::parse("$4.64"),
    );

    ticker.set_cp_data(vec![
        Some(DollarUSD::parse("$700.0")),
        Some(DollarUSD::parse("$725.0")),
        Some(DollarUSD::parse("$750.0")),
        Some(DollarUSD::parse("$775.5")),
        Some(DollarUSD::parse("$700.0")),
        Some(DollarUSD::parse("$809.1")),
        Some(DollarUSD::parse("$800.0")),
        Some(DollarUSD::parse("$800.0")),
    ]);

    let five_days_from_now = now.checked_add_signed(chrono::Duration::days(122)).unwrap();
    let option = Contract::new(
        Box::new(ticker),
        DollarUSD::parse("$805.0"),
        five_days_from_now,
        OptionType::Call,
    );

    let greeks = option.get_all_greeks(Decimal::from_str("0.5").unwrap());
    println!("{}", greeks);
}
