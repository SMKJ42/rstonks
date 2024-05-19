mod decimals;
mod option;
mod stock;

use chrono::Utc;
use decimals::DollarUSD;
use option::{OptionCandidate, OptionType};
use stock::Stock;

use crate::option::greeks::Greek;

fn main() {
    let now = Utc::now();
    let mut ticker = Stock::new();

    ticker.update_price(DollarUSD::parse("$114.0"));
    ticker.set_cp_data(vec![
        Some(DollarUSD::parse("$100.0")),
        Some(DollarUSD::parse("$110.0")),
        Some(DollarUSD::parse("$105.0")),
        Some(DollarUSD::parse("$120.0")),
        Some(DollarUSD::parse("$125.0")),
    ]);

    let five_days_from_now = now.checked_add_signed(chrono::Duration::days(5)).unwrap();
    let option = OptionCandidate::new(
        Box::new(ticker),
        DollarUSD::parse("$115.0"),
        five_days_from_now,
        OptionType::Call,
    );

    let greeks = option.get_all_greeks();

    println!("TTE: {}", option.get_tte());

    for greek in greeks {
        let greek: Greek = greek;
        println!("{}", greek);
    }
}
