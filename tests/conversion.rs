#[cfg(test)]
mod conversion_tests {
    use candid::{Encode, Principal};
    use num_bigint::{ToBigInt, ToBigUint};
    use pretty_assertions::assert_eq;

    use candy::types::{Bytes, Floats, Nats, Property};
    use candy::value::CandyValue;
    use candy::value::ToCandyValue;

    #[test]
    fn conversion_to_nat() {
        let nat = candid::Nat::from(123);
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
        assert_eq!(CandyValue::from(num).to_string(), "12345");

        let num = 123_u8;
        assert_eq!(CandyValue::from(num).to_string(), "123");

        let num = 123_u16;
        assert_eq!(CandyValue::from(num).to_string(), "123");

        let num = 123_u32;
        assert_eq!(CandyValue::from(num).to_string(), "123");

        let num = 123_u64;
        assert_eq!(CandyValue::from(num).to_string(), "123");

        // Float
        let num = 123.12;
        assert_eq!(CandyValue::from(num).to_string(), "123.12");

        let num = 123.868;
        assert_eq!(CandyValue::from(num).to_string(), "123.868");

        let num = -0.001;
        assert_eq!(CandyValue::from(num).to_string(), "-0.001");

        // Int
        let num = -0.51;
        assert_eq!(CandyValue::from(num).to_string(), "-0.51");

        let num = 123_i8;
        assert_eq!(CandyValue::from(num).to_string(), "123");

        let num = -123_i16;
        assert_eq!(CandyValue::from(num).to_string(), "-123");

        let num = -123_i32;
        assert_eq!(CandyValue::from(num).to_string(), "-123");

        let num = -123_i64;
        assert_eq!(CandyValue::from(num).to_string(), "-123");

        // Int
        let num = -123_i128;
        assert_eq!(CandyValue::from(num).to_string(), "-123");

        let num = 123_i8;
        assert_eq!(CandyValue::from(num).to_string(), "123");

        let num = 123_i16;
        assert_eq!(CandyValue::from(num).to_string(), "123");

        let num = 123_i32;
        assert_eq!(CandyValue::from(num).to_string(), "123");

        let num = 123_i64;
        assert_eq!(CandyValue::from(num).to_string(), "123");

        // Text
        let text = "some text".to_string();
        assert_eq!(CandyValue::from(text).to_string(), "some text");

        // Bool
        let bool_candy = CandyValue::from(true);
        let bool_candy_false = CandyValue::from(false);
        assert_eq!(bool_candy.to_string(), "true");
        assert_eq!(bool_candy_false.to_string(), "false");

        //Option
        let option_candy = CandyValue::from(Some(Box::from(CandyValue::from(123))));
        let option_candy_none = CandyValue::from(None);
        assert_eq!(option_candy.to_string(), "123");
        assert_eq!(option_candy_none.to_string(), "null");

        //Blob
        let blob_candy = CandyValue::from(vec![1_u8, 2_u8, 3_u8]);
        let blob_candy_2 = CandyValue::from(vec![15_u8]);
        assert_eq!(blob_candy.to_string(), "010203");
        assert_eq!(blob_candy_2.to_string(), "0f");

        //Bytes
        let bytes_frozen = Bytes::from(vec![1_u8, 2_u8, 3_u8]).to_candy();
        let bytes_thawed = Bytes::thawed(vec![15_u8]).to_candy();
        assert_eq!(bytes_frozen.to_string(), "010203");
        assert_eq!(bytes_thawed.to_string(), "0f");

        //Array
        let array = CandyValue::from(vec![
            CandyValue::from(1),
            CandyValue::from("text"),
            CandyValue::from(3),
        ]);

        assert_eq!(array.to_string(), "[{1} {text} {3}]".to_string());

        //Nats
        let nats = CandyValue::from(vec![123_u128, 1234_u128, 12345_u128]);

        assert_eq!(nats.to_string(), "[123 1234 12345]".to_string());

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
            CandyValue::from(prop).to_string(),
            "{name:some text; name_2:var another text;}".to_string()
        );

        //Principal
        let principal = Principal::anonymous();
        assert_eq!(CandyValue::from(principal).to_string(), "2vxsx-fae");

        //Floats
        let floats = vec![12.35, 25.66].to_candy();

        assert_eq!(
            CandyValue::from(floats).to_string(),
            "[12.35 25.66]".to_string()
        );
    }

    #[test]
    fn conversion_to_blob() {
        // Nat
        let num = 255_u128;
        assert_eq!(num.to_candy().to_blob(), vec![255]);

        // Nat8
        let num = 255_u8;
        assert_eq!(CandyValue::from(num).to_blob(), vec![255]);

        // Nat16
        let num = 2566_u16;
        assert_eq!(CandyValue::from(num).to_blob(), vec![10, 6]);

        // Nat32
        let num = 255_u32;
        assert_eq!(CandyValue::from(num).to_blob(), vec![0, 0, 0, 255]);

        //Nat64
        let num = 300_000_u64;
        assert_eq!(
            CandyValue::from(num).to_blob(),
            vec![0, 0, 0, 0, 0, 4, 147, 224]
        );

        //Int
        let num = -123_i128;
        assert_eq!(CandyValue::from(num).to_blob(), vec![1, 123]);

        // Text
        let text = "some text".to_string();
        assert_eq!(
            CandyValue::from(text).to_blob(),
            vec![115, 111, 109, 101, 32, 116, 101, 120, 116,]
        );

        //Blob
        let blob = CandyValue::from("some_text".to_string()).to_blob();
        let text = String::from_utf8(blob);
        assert_eq!(text.unwrap(), "some_text".to_string());

        //Bytes
        let bytes_frozen = Bytes::from(vec![1_u8, 2_u8, 3_u8]).to_candy();
        let bytes_thawed = Bytes::thawed(vec![1_u8, 2_u8, 3_u8]).to_candy();
        assert_eq!(bytes_frozen.to_blob(), vec![1_u8, 2_u8, 3_u8]);
        assert_eq!(bytes_thawed.to_blob(), vec![1_u8, 2_u8, 3_u8]);

        //Principal
        let principal = Principal::anonymous();
        let result = CandyValue::from(principal).to_blob();
        assert_eq!(result, vec![4]);
    }

    #[test]
    fn conversion_to_json() {
        // Nat
        assert_eq!(CandyValue::from(123_u128).to_json(), "123");
        assert_eq!(CandyValue::from(123_u64).to_json(), "123");
        assert_eq!(CandyValue::from(123_u64).to_json(), "123");
        assert_eq!(CandyValue::from(123_u32).to_json(), "123");
        assert_eq!(CandyValue::from(123_u16).to_json(), "123");
        assert_eq!(CandyValue::from(123_u8).to_json(), "123");
        assert_eq!(CandyValue::from("text").to_json(), "\"text\"");
        assert_eq!(CandyValue::from("text").to_json(), "\"text\"");
        assert_eq!(
            CandyValue::from(vec![
                Property {
                    value: CandyValue::from(123_u8),
                    name: "test".to_string(),
                    immutable: true
                },
                Property {
                    value: CandyValue::from(124_u8),
                    name: "test_2".to_string(),
                    immutable: false
                }
            ])
            .to_json(),
            "{\"test\":123,\"test_2\":124}"
        );
        assert_eq!(
            vec![1.to_candy(), 2.to_candy()].to_candy().to_json(),
            "[1,2]"
        );

        assert_eq!(
            Some(Box::from(123_u128.to_candy())).to_candy().to_json(),
            "123"
        );
        assert_eq!(CandyValue::from(None).to_json(), "null");

        assert_eq!(
            candid::Principal::management_canister()
                .to_candy()
                .to_json(),
            "\"aaaaa-aa\""
        );

        let nats_frozen = vec![123_u128, 1234_u128, 12345_u128].to_candy();
        let nats_thawed = CandyValue::Nats(Nats::thawed(vec![123_u128, 1234_u128, 12345_u128]));
        assert_eq!(nats_frozen.to_json(), "[123,1234,12345]");
        assert_eq!(nats_thawed.to_json(), "[123,1234,12345]");

        //Floats
        let floats_frozen = vec![12.35, 25.66].to_candy();
        let floats_thawed = CandyValue::Floats(Floats::thawed(vec![12.35, 25.66]));
        assert_eq!(floats_frozen.to_json(), "[12.35,25.66]");
        assert_eq!(floats_thawed.to_json(), "[12.35,25.66]");

        //Bytes
        let bytes_frozen = Bytes::from(vec![1_u8, 2_u8, 3_u8]).to_candy();
        let bytes_thawed = Bytes::thawed(vec![15_u8]).to_candy();
        assert_eq!(bytes_frozen.to_json(), "\"010203\"");
        assert_eq!(bytes_thawed.to_json(), "\"0f\"");

        //Blob
        let blob = CandyValue::from(vec![1_u8, 2_u8, 3_u8]);
        assert_eq!(blob.to_json(), "\"010203\"");

        //Principal
        let principal = Principal::anonymous();
        let result = CandyValue::from(principal).to_json();
        assert_eq!(result, "\"2vxsx-fae\"");

        //Bool
        let t = true.to_candy();
        let f = false.to_candy();
        assert_eq!(t.to_json(), "\"true\"");
        assert_eq!(f.to_json(), "\"false\"");

        //Float
        let f = 12.35.to_candy();
        assert_eq!(f.to_json(), "12.35");

        //Empty
        assert_eq!(CandyValue::Empty.to_json(), "null");

        //Int
        let num = 123_i128.to_candy();
        assert_eq!(num.to_json(), "123");
        //Int64
        let num = 123_i64.to_candy();
        assert_eq!(num.to_json(), "123");
        //Int32
        let num = 123_i32.to_candy();
        assert_eq!(num.to_json(), "123");
        //Int16
        let num = 123_i16.to_candy();
        assert_eq!(num.to_json(), "123");
        //Int8
        let num = 123_i8.to_candy();
        assert_eq!(num.to_json(), "123");
    }
}
