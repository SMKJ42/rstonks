use super::OptionType;
use core::fmt;
use decimal_rs::Decimal;
use statrs::distribution::{Continuous, ContinuousCDF, Normal};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

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

impl From<[Greek; 5]> for Greeks {
    fn from(greeks: [Greek; 5]) -> Self {
        Greeks { greeks }
    }
}

impl Display for Greeks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Delta: {}\nGamma: {}\nTheta: {}\nVega: {}\nRho: {}",
            self.greeks[0], self.greeks[1], self.greeks[2], self.greeks[3], self.greeks[4]
        )
    }
}

impl Iterator for Greeks {
    type Item = Greek;

    fn next(&mut self) -> Option<Self::Item> {
        for greek in self.greeks.iter() {
            return Some(*greek);
        }
        return None;
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

impl fmt::Display for Greek {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Greek::Delta(e) => write!(f, "{}", e),
            Greek::Gamma(e) => write!(f, "{}", e),
            Greek::Theta(e) => write!(f, "{}", e),
            Greek::Vega(e) => write!(f, "{}", e),
            Greek::Rho(e) => write!(f, "{}", e),
        }
    }
}

// s: Current price of the underlying asset
// k: Strike price of the option
// t: Time to expiration (in years)
// r: Risk-free interest rate
// v: Volatility of the underlying asset

pub fn get_all_greeks(
    c_type: OptionType,
    s: Decimal,
    k: Decimal,
    t: Decimal,
    r: Decimal,
    v: Decimal,
) -> Greeks {
    let d1: Decimal = ((s / k).ln().unwrap() + (r + Decimal::from_str("0.5").unwrap() * v * v) * t)
        / (v * t.sqrt().unwrap());
    let d2 = d1 - v * t.sqrt().unwrap();

    let normal = Normal::new(0.0, 1.0).unwrap();
    let nd1 = normal.cdf(d1.into());
    let nd2 = normal.cdf(d2.into());
    let npd1 = normal.pdf(d1.into());

    let delta: Decimal;
    let theta: Decimal;
    let rho: Decimal;

    if c_type == OptionType::Call {
        delta = Decimal::from(0) + nd1;
        theta = -(s * v * npd1) / (2.0 * t.sqrt().unwrap())
            - r * k * Decimal::exp(&(-r * t)).unwrap() * 3.125;
        rho = k * t * Decimal::exp(&(-r * t)).unwrap() * nd2;
    } else if c_type == OptionType::Put {
        delta = Decimal::from(0) - nd1;
        theta = -(s * v * npd1) / (2.0 * t.sqrt().unwrap())
            + r * k * Decimal::exp(&(-r * t)).unwrap() * 3.125;
        rho = -k * t * Decimal::exp(&(-r * t)).unwrap() * nd2;
    } else {
        panic!("Invalid option type");
    }

    let gamma = npd1 / (s * v * t.sqrt().unwrap());

    let vega = s * t.sqrt().unwrap() * npd1;

    return Greeks::from([
        Greek::Delta(delta),
        Greek::Gamma(gamma),
        Greek::Theta(theta),
        Greek::Vega(vega),
        Greek::Rho(rho),
    ]);
}
