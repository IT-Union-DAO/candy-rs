#[cfg(test)]
mod conversion_tests {
    use candid::Principal;
    use pretty_assertions::assert_eq;

    use candy::conversion::imp::UnboxCandyValue;
    use candy::types::types::Property;
    use candy::types::value::CandyValue;

    #[test]
    fn conversion_to_nat() {
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
        let neg_num_near_zero = -0.001;
        let neg_num_near_one = -0.51;
        println!(
            "f64 (Float) conversions - {:?}",
            [num, num_2, neg_num_near_zero]
        );
        let f64_candy = CandyValue::from(num);
        let f64_candy_2 = CandyValue::from(num_2);
        let f64_candy_neg = CandyValue::from(neg_num_near_zero);
        let f64_candy_neg_near_one = CandyValue::from(neg_num_near_one);
        assert_eq!(f64_candy.to_nat().unwrap(), 123_u128);
        assert_eq!(f64_candy_2.to_nat().unwrap(), 124_u128);
        assert_eq!(f64_candy_neg.to_nat(), None);
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

    #[test]
    fn conversion_to_string() {
        // Nat
        let num = 12345_u128;
        assert_eq!(CandyValue::from(num).to_text(), "12345");

        let num = 123_u8;
        assert_eq!(CandyValue::from(num).to_text(), "123");

        let num = 123_u16;
        assert_eq!(CandyValue::from(num).to_text(), "123");

        let num = 123_u32;
        assert_eq!(CandyValue::from(num).to_text(), "123");

        let num = 123_u64;
        assert_eq!(CandyValue::from(num).to_text(), "123");

        // Float
        let num = 123.12;
        assert_eq!(CandyValue::from(num).to_text(), "123.12");

        let num = 123.868;
        assert_eq!(CandyValue::from(num).to_text(), "123.868");

        let num = -0.001;
        assert_eq!(CandyValue::from(num).to_text(), "-0.001");

        // Int
        let num = -0.51;
        assert_eq!(CandyValue::from(num).to_text(), "-0.51");

        let num = 123_i8;
        assert_eq!(CandyValue::from(num).to_text(), "123");

        let num = -123_i16;
        assert_eq!(CandyValue::from(num).to_text(), "-123");

        let num = -123_i32;
        assert_eq!(CandyValue::from(num).to_text(), "-123");

        let num = -123_i64;
        assert_eq!(CandyValue::from(num).to_text(), "-123");

        // Int
        let num = -123_i128;
        assert_eq!(CandyValue::from(num).to_text(), "-123");

        let num = 123_i8;
        assert_eq!(CandyValue::from(num).to_text(), "123");

        let num = 123_i16;
        assert_eq!(CandyValue::from(num).to_text(), "123");

        let num = 123_i32;
        assert_eq!(CandyValue::from(num).to_text(), "123");

        let num = 123_i64;
        assert_eq!(CandyValue::from(num).to_text(), "123");

        // Text
        let text = "some text".to_string();
        assert_eq!(CandyValue::from(text).to_text(), "some text");

        // Bool
        let bool_candy = CandyValue::from(true);
        let bool_candy_false = CandyValue::from(false);
        assert_eq!(bool_candy.to_text(), "true");
        assert_eq!(bool_candy_false.to_text(), "false");

        //Option
        let option_candy = CandyValue::from(Some(Box::from(CandyValue::from(123))));
        let option_candy_none = CandyValue::from(None);
        assert_eq!(option_candy.to_text(), "123");
        assert_eq!(option_candy_none.to_text(), "null");

        //Blob
        let blob_candy = CandyValue::from(vec![1_u8, 2_u8, 3_u8]);
        let blob_candy_2 = CandyValue::from(vec![15_u8]);
        assert_eq!(blob_candy.to_text(), "010203");
        assert_eq!(blob_candy_2.to_text(), "0f");

        //Array
        let array = CandyValue::from(vec![
            CandyValue::from(1),
            CandyValue::from("text"),
            CandyValue::from(3),
        ]);

        assert_eq!(array.to_text(), "[{1} {text} {3}]".to_string());

        //Nats
        let nats = CandyValue::from(vec![123_u128, 1234_u128, 12345_u128]);

        assert_eq!(nats.to_text(), "[123 1234 12345]".to_string());

        //Class
        let prop = vec![
            Property {
                name: "name".to_string(),
                value: CandyValue::from("some text".to_string()),
                immutable: false,
            },
            Property {
                name: "name_2".to_string(),
                value: CandyValue::from("another text".to_string()),
                immutable: true,
            },
        ];
        assert_eq!(
            CandyValue::from(prop).to_text(),
            "{name:some text; name_2:var another text;}".to_string()
        );

        //Principal
        let principal = Principal::anonymous();
        assert_eq!(CandyValue::from(principal).to_text(), "2vxsx-fae");

        //Floats
        let floats = CandyValue::from(vec![12.35, 25.66]);

        assert_eq!(
            CandyValue::from(floats).to_text(),
            "[12.35 25.66]".to_string()
        );
    }

    #[test]
    fn conversion_to_blob() {
        // Nat
        let num = 255_u128;
        assert_eq!(CandyValue::from(num).to_blob().unwrap(), vec![24, 255]);

        // Nat8
        let num = 255_u8;
        assert_eq!(CandyValue::from(num).to_blob().unwrap(), vec![24, 255]);

        // Nat16
        let num = 2566_u16;
        assert_eq!(CandyValue::from(num).to_blob().unwrap(), vec![25, 10, 6]);

        // Nat32
        let num = 255_u32;
        assert_eq!(CandyValue::from(num).to_blob().unwrap(), vec![24, 255]);

        //Nat64
        let num = 300_000_u64;
        assert_eq!(
            CandyValue::from(num).to_blob().unwrap(),
            vec![26, 0, 4, 147, 224]
        );

        //Int
        let num = -123_i128;
        assert_eq!(CandyValue::from(num).to_blob().unwrap(), vec![56, 122]);

        //Int8
        let num = -123_i8;
        assert_eq!(CandyValue::from(num).to_blob().unwrap(), vec![56, 122]);

        //Int16
        let num = -123_i16;
        assert_eq!(CandyValue::from(num).to_blob().unwrap(), vec![56, 122]);

        //Int32
        let num = -123_i32;
        assert_eq!(CandyValue::from(num).to_blob().unwrap(), vec![56, 122]);

        //Float
        let num = -123.12;
        assert_eq!(
            CandyValue::from(num).to_blob().unwrap(),
            vec![251, 192, 94, 199, 174, 20, 122, 225, 72]
        );

        // Text
        let text = "some text".to_string();
        assert_eq!(
            CandyValue::from(text).to_blob().unwrap(),
            vec![115, 111, 109, 101, 32, 116, 101, 120, 116,]
        );

        let blob = CandyValue::from("some_text".to_string()).to_blob().unwrap();
        let text = String::from_utf8(blob).unwrap();
        assert_eq!(text, "some_text".to_string());

        //Principal
        let principal = Principal::anonymous();
        let result = CandyValue::from(principal).to_blob();
        assert_eq!(result.unwrap(), vec![4]);

        //Bool
        let t = true;
        let f = false;
        assert_eq!(CandyValue::from(t).to_blob().unwrap(), vec![245]);
        assert_eq!(CandyValue::from(f).to_blob().unwrap(), vec![244])
    }
}
