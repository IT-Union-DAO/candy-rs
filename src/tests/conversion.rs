#[cfg(test)]
mod conversion_tests {
    use pretty_assertions::assert_eq;

    use crate::conversion::imp::UnboxCandyValue;
    use crate::stable::value::CandyValue;

    #[test]
    fn test_conversion_to_nat() {
        let num = 12345_u128;
        println!("u128 (Nat) conversion - {:?}", num);
        let nat_candy = CandyValue::from(num);
        assert_eq!(nat_candy.to_nat().unwrap(), num);

        let num = 123_u8;
        println!("u8 (Nat8) conversion - {:?}", num);
        let nat8_candy = CandyValue::from(num);
        assert_eq!(nat8_candy.to_nat().unwrap(), 123_u128);

        let num = 123_u16;
        println!("u16 (Nat16) conversion - {:?}", num);
        let nat16_candy = CandyValue::from(num);
        assert_eq!(nat16_candy.to_nat().unwrap(), 123_u128);

        let num = 123_u32;
        println!("u32 (Nat32) conversion - {:?}", num);
        let nat32_candy = CandyValue::from(num);
        assert_eq!(nat32_candy.to_nat().unwrap(), 123_u128);


        let num = 123_u64;
        println!("u64 (Nat64) conversion - {:?}", num);
        let nat64_candy = CandyValue::from(num);
        assert_eq!(nat64_candy.to_nat().unwrap(), 123_u128);


        let num = 123.12;
        let num_2 = 123.868;
        let neg_num_near_zero = -0.001; //TODO  will be rounded to zero, is it ok?
        let neg_num_near_one = -0.51;
        println!("f64 (Float) conversions - {:?}", [num, num_2, neg_num_near_zero]);
        let f64_candy = CandyValue::from(num);
        let f64_candy_2 = CandyValue::from(num_2);
        let f64_candy_neg = CandyValue::from(neg_num_near_zero);
        let f64_candy_neg_near_one = CandyValue::from(neg_num_near_one);
        assert_eq!(f64_candy.to_nat().unwrap(), 123_u128);
        assert_eq!(f64_candy_2.to_nat().unwrap(), 124_u128);
        assert_eq!(f64_candy_neg.to_nat().unwrap(), 0_u128);
        assert_eq!(f64_candy_neg_near_one.to_nat(), None);

        let num = 123_i128;
        let num_neg = -12_i128;

        println!("i128 (Int) conversions - {:?}", [num, num_neg]);
        let i128_candy = CandyValue::from(num);
        let i128_candy_neg = CandyValue::from(num_neg);
        assert_eq!(i128_candy.to_nat().unwrap(), 123_u128);
        assert_eq!(i128_candy_neg.to_nat(), None);


        let num = 127_i8;
        let num_neg = -128_i8;
        println!("i8 (Int8) conversions - {:?}", [num, num_neg]);
        let i8_candy = CandyValue::from(num);
        let i8_candy_neg = CandyValue::from(num_neg);
        assert_eq!(i8_candy.to_nat().unwrap(), 127_u128);
        assert_eq!(i8_candy_neg.to_nat(), None);


        let num = 127_i16;
        let num_neg = -128_i16;
        println!("i16 (Int16) conversions - {:?}", [num, num_neg]);
        let i16_candy = CandyValue::from(num);
        let i16_candy_neg = CandyValue::from(num_neg);
        assert_eq!(i16_candy.to_nat().unwrap(), 127_u128);
        assert_eq!(i16_candy_neg.to_nat(), None);

        let num = 127_i32;
        let num_neg = -128_i32;
        println!("i32 (Int32) conversions - {:?}", [num, num_neg]);
        let i32_candy = CandyValue::from(num);
        let i32_candy_neg = CandyValue::from(num_neg);
        assert_eq!(i32_candy.to_nat().unwrap(), 127_u128);
        assert_eq!(i32_candy_neg.to_nat(), None);


        let num = 127_i64;
        let num_neg = -128_i64;
        println!("i64 (Int64) conversions - {:?}", [num, num_neg]);
        let i64_candy = CandyValue::from(num);
        let i64_candy_neg = CandyValue::from(num_neg);
        assert_eq!(i64_candy.to_nat().unwrap(), 127_u128);
        assert_eq!(i64_candy_neg.to_nat(), None);
    }
}