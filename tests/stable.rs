#[cfg(test)]
mod stable_types_tests {
    use pretty_assertions::assert_eq;

    use candy::unstable::value::CandyValueUnstable;
    use std::str::FromStr;

    #[test]
    fn create_stable() {
        let unstable_nat = CandyValueUnstable::from(1234 as u128);
        assert_eq!(unstable_nat, CandyValueUnstable::Nat(1234));
    }
}