#[cfg(test)]
mod tests {

    use rust_decimal::Decimal;

    use crate::decimals::DollarUSD;

    #[test]
    fn parse() {
        let a = DollarUSD::parse("$1.0");
        let b = DollarUSD::new(Decimal::from(1));
        assert_eq!(a.get_dollars(), b.get_dollars());
    }

    #[test]
    fn addition() {
        let a = DollarUSD::parse("$1.0");
        let b = DollarUSD::parse("$2.0");
        assert_eq!((a + b), DollarUSD::parse("$3.0"));
    }

    #[test]
    fn subtraction() {
        let a = DollarUSD::parse("$3.0");
        let b = DollarUSD::parse("$2.0");
        assert_eq!(a - b, DollarUSD::parse("$1.0"));
    }

    #[test]
    fn multiplication() {
        let a = DollarUSD::parse("$3.00");
        let b = DollarUSD::parse("$2.00");
        assert_eq!(a * b, DollarUSD::parse("$6.00"));
    }

    #[test]
    fn division() {
        let a = DollarUSD::parse("$6.0");
        let b = DollarUSD::parse("$2.0");
        assert_eq!(a / b, DollarUSD::parse("$3.0"));
    }

    #[test]
    fn division_round_down() {
        let a = DollarUSD::parse("$5.35");
        let b = DollarUSD::parse("$3.0");
        assert_eq!(
            (a / b).get_dollars(),
            DollarUSD::new(Decimal::from_str_exact("1.78").unwrap())
        );
    }

    #[test]
    fn division_round_up() {
        let a = DollarUSD::parse("$5.75");
        let b = DollarUSD::parse("$2.0");
        assert_eq!(
            (a / b).get_dollars(),
            DollarUSD::new(Decimal::from_str_exact("2.88").unwrap())
        );
    }
}
