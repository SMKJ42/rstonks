use rust_decimal::{Decimal, MathematicalOps};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, Mul, Sub};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct DollarUSD {
    value: Decimal,
}

impl Display for DollarUSD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "${}", self.value.round_dp(2))
    }
}

impl DollarUSD {
    pub fn new(value: Decimal) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> Decimal {
        self.value
    }

    pub fn get_dollars(&self) -> DollarUSD {
        return DollarUSD::new(self.value.round_dp(2));
    }

    //TODO: create function for getting DollarUSD as fractional cents (e.g. $1.255)

    pub fn parse(s: &str) -> Self {
        let value = s.replace("$", "");
        let value = Decimal::from_str(value.as_str()).unwrap();
        return Self::new(value);
    }

    pub fn ln(self) -> Self {
        Self {
            value: self.value.ln(),
        }
    }

    pub fn get_decimal(self) -> Decimal {
        self.value
    }
}

impl Add for DollarUSD {
    type Output = DollarUSD;

    fn add(self, rhs: Self) -> Self::Output {
        DollarUSD {
            value: self.value + rhs.value,
        }
    }
}

impl Sub for DollarUSD {
    type Output = DollarUSD;

    fn sub(self, other: DollarUSD) -> DollarUSD {
        DollarUSD {
            value: self.value - other.value,
        }
    }
}

impl Mul for DollarUSD {
    type Output = DollarUSD;

    fn mul(self, other: DollarUSD) -> DollarUSD {
        DollarUSD {
            value: self.value * other.value,
        }
    }
}

impl Div for DollarUSD {
    type Output = DollarUSD;

    fn div(self, other: DollarUSD) -> DollarUSD {
        DollarUSD {
            value: self.value / other.value,
        }
    }
}

impl AddAssign for DollarUSD {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}
