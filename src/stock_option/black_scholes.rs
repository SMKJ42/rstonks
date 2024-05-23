use crate::{decimals::DollarUSD, stock_option::ContractType};
use core::fmt;
use rust_decimal::{Decimal, MathematicalOps};
use std::fmt::{Debug, Display};

pub type Greeks = [Greek; 5];
#[derive(PartialEq, Clone)]
pub struct BlackScholes {
    pub greeks: Greeks,
}

impl BlackScholes {
    // s: Current price of the underlying asset
    // k: Strike price of the option
    // t: Time to expiration (in years)
    // r: Risk-free interest rate
    // v: Volatility of the underlying asset
    // d: Dividend yield of the underlying asset
    // rf_rate: Risk-free interest rate
    pub fn greeks(
        s: Decimal,
        k: Decimal,
        t: Decimal,
        v: Decimal,
        d: Decimal,
        rf_rate: Decimal,
        o_type: ContractType,
    ) -> BlackScholes {
        println!(
            "s: {}, k: {}, t: {}, v: {}, d: {}, rf_rate: {}",
            s, k, t, v, d, rf_rate
        );
        let d1: Decimal = ((s / k).ln() + (rf_rate - d + (v * v) / Decimal::from(2)) * t)
            / (v * t.sqrt().unwrap());
        let d2 = d1 - v * t.sqrt().unwrap();

        let nd1 = d1.norm_cdf();
        let nd2 = d2.norm_cdf();
        let npd1 = d1.norm_pdf();

        let rel_d = Decimal::exp(&(-(d / s) * t));
        let rel_r = Decimal::exp(&(-rf_rate * t));

        let delta: Decimal;
        let mut theta: Decimal;
        let mut rho: Decimal;

        match o_type {
            ContractType::Call => {
                delta = rel_d * nd1;
                rho = k * t * rel_r * d2;
                let rho_ish = rf_rate * k * rel_r * nd2;
                theta = -(rel_d * s * v * npd1) / (Decimal::TWO * t.sqrt().unwrap())
                    - d * s * rel_d * nd1
                    + rho_ish;
            }
            ContractType::Put => {
                delta = rel_d * (Decimal::ONE - nd1);
                rho = -k * t * rel_r * (Decimal::ONE - d2);
                let rho_ish = rf_rate * k * rel_r * (Decimal::ONE - nd2);
                theta = -(rel_d * s * v * npd1) / (Decimal::TWO * t.sqrt().unwrap())
                    + d * s * rel_d * (Decimal::ONE - nd1)
                    - rho_ish;
            }
        }

        let gamma = (rel_d * npd1) / (s * v * t.sqrt().unwrap());
        let mut vega = s * t.sqrt().unwrap() * npd1 * rel_d;

        theta = theta / Decimal::from(360);
        rho = rho / Decimal::ONE_HUNDRED;
        vega = vega / Decimal::ONE_HUNDRED;

        return BlackScholes {
            greeks: [
                Greek::Delta(delta),
                Greek::Gamma(gamma),
                Greek::Theta(theta),
                Greek::Vega(vega),
                Greek::Rho(rho),
            ],
        };
    }

    // s: Current price of the underlying asset
    // k: Strike price of the option
    // t: Time to expiration (in years)
    // r: Risk-free interest rate
    // v: Volatility of the underlying asset
    // d: Dividend yield of the underlying asset
    // rf_rate: Risk-free interest rate
    pub fn price_option(
        k: Decimal,
        t: Decimal,
        s: Decimal,
        v: Decimal,
        d: Decimal,
        rf_rate: Decimal,
        o_type: ContractType,
    ) -> DollarUSD {
        let d1: Decimal = ((s / k).ln() + (rf_rate - d + (v * v) / Decimal::from(2)) * t)
            / (v * t.sqrt().unwrap());
        let d2 = d1 - v * t.sqrt().unwrap();
        let nd1 = d1.norm_cdf();
        let nd2 = d2.norm_cdf();

        let rel_d = Decimal::exp(&(-(&d / s) * t));
        let rel_r = Decimal::exp(&(-rf_rate * t));

        match o_type {
            ContractType::Call => {
                let val = s * nd1 * rel_d - k * rel_r * nd2;
                return DollarUSD::new(val);
            }
            ContractType::Put => {
                let val = k * rel_r * (Decimal::ONE - nd2) - s * (Decimal::ONE - nd1) * rel_d;
                return DollarUSD::new(val);
            }
        }
    }
}

impl IntoIterator for BlackScholes {
    type Item = Greek;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.greeks.to_vec().into_iter()
    }
}

impl From<[Greek; 5]> for BlackScholes {
    fn from(greeks: [Greek; 5]) -> Self {
        BlackScholes { greeks }
    }
}

impl Display for BlackScholes {
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

impl Debug for BlackScholes {
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
