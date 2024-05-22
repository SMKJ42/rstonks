use super::OptionType;
use crate::decimals::DollarUSD;
use core::fmt;
use rust_decimal::{Decimal, MathematicalOps};
use std::fmt::{Debug, Display};

#[derive(PartialEq, Clone)]
pub struct Greeks {
    pub greeks: [Greek; 5],
}

impl Greeks {
    pub fn new() -> Greeks {
        Greeks {
            greeks: [
                Greek::Delta(Decimal::from(0)),
                Greek::Gamma(Decimal::from(0)),
                Greek::Theta(Decimal::from(0)),
                Greek::Vega(Decimal::from(0)),
                Greek::Rho(Decimal::from(0)),
            ],
        }
    }
}

impl IntoIterator for Greeks {
    type Item = Greek;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.greeks.to_vec().into_iter()
    }
}

impl From<[Greek; 5]> for Greeks {
    fn from(greeks: [Greek; 5]) -> Self {
        Greeks { greeks }
    }
}

impl Display for Greeks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Delta      Gamma      Theta      Vega       Rho       \n"
        )?;
        write!(
            f,
            "{:<10.4} {:<10.4} {:<10.4} {:<10.4} {:<10.4}",
            self.greeks[0].get_value(),
            self.greeks[1].get_value(),
            self.greeks[2].get_value(),
            self.greeks[3].get_value(),
            self.greeks[4].get_value(),
        )
    }
}

impl Debug for Greeks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Delta: {:?}\nGamma: {:?}\nTheta: {:?}\nVega: {:?}\nRho: {:?}",
            self.greeks[0], self.greeks[1], self.greeks[2], self.greeks[3], self.greeks[4]
        )
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Greek {
    Delta(Decimal),
    Gamma(Decimal),
    Theta(Decimal),
    Vega(Decimal),
    Rho(Decimal),
}

impl Greek {
    fn get_value(&self) -> Decimal {
        match self {
            Greek::Delta(e) => *e,
            Greek::Gamma(e) => *e,
            Greek::Theta(e) => *e,
            Greek::Vega(e) => *e,
            Greek::Rho(e) => *e,
        }
    }
    fn get_name(&self) -> String {
        match self {
            Greek::Delta(_) => "Delta",
            Greek::Gamma(_) => "Gamma",
            Greek::Theta(_) => "Theta",
            Greek::Vega(_) => "Vega",
            Greek::Rho(_) => "Rho",
        }
        .to_string()
    }
}

// s: Current price of the underlying asset
// k: Strike price of the option
// t: Time to expiration (in years)
// r: Risk-free interest rate
// v: Volatility of the underlying asset

pub fn get_all_greeks(
    o_type: OptionType,
    s: Decimal,
    k: Decimal,
    t: Decimal,
    r: Decimal,
    v: Decimal,
    d: DollarUSD,
) -> Greeks {
    let d1: Decimal =
        ((s / k).ln() + (r + (v * v) / Decimal::from(2)) * t) / (v * t.sqrt().unwrap());
    let d2 = d1 - v * t.sqrt().unwrap();

    let nd1 = d1.norm_cdf();
    // !!! if pdf overflow happens, its here
    let npd1 = d1.norm_pdf();

    let rel_d = Decimal::exp(&(-(d.get_decimal() / s) * t));
    let rel_r = Decimal::exp(&(-r * t));

    let delta: Decimal;
    let mut theta: Decimal;
    let mut rho: Decimal;

    match o_type {
        OptionType::Call => {
            let nd2 = d2.norm_cdf();
            delta = rel_d * nd1;
            rho = k * t * rel_r * nd2;
            let rho_ish = r * k * rel_r * nd2;
            theta = -(rel_d * s * v * npd1) / (Decimal::from(2) * t.sqrt().unwrap()) - rho_ish
                + d.get_decimal() * rel_d * nd1;
        }
        OptionType::Put => {
            delta = rel_d * (nd1 - Decimal::from(1));
            rho = -k * t * rel_r * (-d2).norm_cdf();
            let rho_ish = r * k * rel_r * (-d2).norm_cdf();
            theta = -(rel_d * s * v * npd1) / (Decimal::from(2) * t.sqrt().unwrap()) + rho_ish
                - d.get_decimal() * rel_d * (-d1.norm_cdf());
        }
        _ => panic!("Invalid option type"),
    }

    theta = theta / Decimal::from(365);
    rho = rho / Decimal::from(100);

    let gamma = (rel_d * npd1) / (s * v * t.sqrt().unwrap());
    let vega = s * t.sqrt().unwrap() * npd1 * rel_d;

    return Greeks::from([
        Greek::Delta(delta),
        Greek::Gamma(gamma),
        Greek::Theta(theta),
        Greek::Vega(vega / Decimal::from(100)),
        Greek::Rho(rho),
    ]);
}
