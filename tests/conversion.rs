#![allow(unused_imports)]
#[cfg(test)]
mod conversion_tests {
    use candid::Principal;
    use pretty_assertions::assert_eq;

    use ic_candy::types::PropertyShared;
    use ic_candy::value::CandyShared::Bytes;
    use ic_candy::value::ToCandyValue;
    use ic_candy::value::{CandyShared, ToBlob};

    #[test]
    fn conversion_to_nat() {
        let num = 12345_u128;
        println!("u128 (Nat) conversion - {:?}", num);
        let nat_candy = CandyShared::from(num);
        assert_eq!(nat_candy.to_nat().unwrap(), num);

        let num = 123_u8;
        println!("u8 (Nat8) conversion - {:?}", num);
        let nat8_candy = CandyShared::from(num);
        assert_eq!(nat8_candy.to_nat().unwrap(), 123_u128);

        let num = 123_u16;
        println!("u16 (Nat16) conversion - {:?}", num);
        let nat16_candy = CandyShared::from(num);
        assert_eq!(nat16_candy.to_nat().unwrap(), 123_u128);

        let num = 123_u32;
        println!("u32 (Nat32) conversion - {:?}", num);
        let nat32_candy = CandyShared::from(num);
        assert_eq!(nat32_candy.to_nat().unwrap(), 123_u128);

        let num = 123_u64;
        println!("u64 (Nat64) conversion - {:?}", num);
        let nat64_candy = CandyShared::from(num);
        assert_eq!(nat64_candy.to_nat().unwrap(), 123_u128);

        let num = 123.12;
        let num_2 = 123.868;
        let neg_num_near_zero = -0.001;
        let neg_num_near_one = -0.51;
        println!(
            "f64 (Float) conversions - {:?}",
            [num, num_2, neg_num_near_zero]
        );
        let f64_candy = CandyShared::from(num);
        let f64_candy_2 = CandyShared::from(num_2);
        let f64_candy_neg = CandyShared::from(neg_num_near_zero);
        let f64_candy_neg_near_one = CandyShared::from(neg_num_near_one);
        assert_eq!(f64_candy.to_nat().unwrap(), 123_u128);
        assert_eq!(f64_candy_2.to_nat().unwrap(), 124_u128);
        assert_eq!(f64_candy_neg.to_nat(), None);
        assert_eq!(f64_candy_neg_near_one.to_nat(), None);

        let num = 123_i128;
        let num_neg = -12_i128;

        println!("i128 (Int) conversions - {:?}", [num, num_neg]);
        let i128_candy = CandyShared::from(num);
        let i128_candy_neg = CandyShared::from(num_neg);
        assert_eq!(i128_candy.to_nat().unwrap(), 123_u128);
        assert_eq!(i128_candy_neg.to_nat(), None);

        let num = 127_i8;
        let num_neg = -128_i8;
        println!("i8 (Int8) conversions - {:?}", [num, num_neg]);
        let i8_candy = CandyShared::from(num);
        let i8_candy_neg = CandyShared::from(num_neg);
        assert_eq!(i8_candy.to_nat().unwrap(), 127_u128);
        assert_eq!(i8_candy_neg.to_nat(), None);

        let num = 127_i16;
        let num_neg = -128_i16;
        println!("i16 (Int16) conversions - {:?}", [num, num_neg]);
        let i16_candy = CandyShared::from(num);
        let i16_candy_neg = CandyShared::from(num_neg);
        assert_eq!(i16_candy.to_nat().unwrap(), 127_u128);
        assert_eq!(i16_candy_neg.to_nat(), None);

        let num = 127_i32;
        let num_neg = -128_i32;
        println!("i32 (Int32) conversions - {:?}", [num, num_neg]);
        let i32_candy = CandyShared::from(num);
        let i32_candy_neg = CandyShared::from(num_neg);
        assert_eq!(i32_candy.to_nat().unwrap(), 127_u128);
        assert_eq!(i32_candy_neg.to_nat(), None);

        let num = 127_i64;
        let num_neg = -128_i64;
        println!("i64 (Int64) conversions - {:?}", [num, num_neg]);
        let i64_candy = CandyShared::from(num);
        let i64_candy_neg = CandyShared::from(num_neg);
        assert_eq!(i64_candy.to_nat().unwrap(), 127_u128);
        assert_eq!(i64_candy_neg.to_nat(), None);
    }

    #[test]
    fn conversion_to_string() {
        // Nat
        let num = 12345_u128;
        assert_eq!(CandyShared::from(num).to_string(), "12345");

        let num = 123_u8;
        assert_eq!(CandyShared::from(num).to_string(), "123");

        let num = 123_u16;
        assert_eq!(CandyShared::from(num).to_string(), "123");

        let num = 123_u32;
        assert_eq!(CandyShared::from(num).to_string(), "123");

        let num = 123_u64;
        assert_eq!(CandyShared::from(num).to_string(), "123");

        // Float
        let num = 123.12;
        assert_eq!(CandyShared::from(num).to_string(), "123.12");

        let num = 1.0;
        assert_eq!(CandyShared::from(num).to_string(), "1");

        let num = 123.868;
        assert_eq!(CandyShared::from(num).to_string(), "123.868");

        let num = -0.001;
        assert_eq!(CandyShared::from(num).to_string(), "-0.001");

        // Int
        let num = -0.51;
        assert_eq!(CandyShared::from(num).to_string(), "-0.51");

        let num = 123_i8;
        assert_eq!(CandyShared::from(num).to_string(), "123");

        let num = -123_i16;
        assert_eq!(CandyShared::from(num).to_string(), "-123");

        let num = -123_i32;
        assert_eq!(CandyShared::from(num).to_string(), "-123");

        let num = -123_i64;
        assert_eq!(CandyShared::from(num).to_string(), "-123");

        // Int
        let num = -123_i128;
        assert_eq!(CandyShared::from(num).to_string(), "-123");

        let num = 123_i8;
        assert_eq!(CandyShared::from(num).to_string(), "123");

        let num = 123_i16;
        assert_eq!(CandyShared::from(num).to_string(), "123");

        let num = 123_i32;
        assert_eq!(CandyShared::from(num).to_string(), "123");

        let num = 123_i64;
        assert_eq!(CandyShared::from(num).to_string(), "123");

        // Text
        let text = "some text".to_string();
        assert_eq!(CandyShared::from(text).to_string(), "some text");

        // Bool
        let bool_candy = CandyShared::from(true);
        let bool_candy_false = CandyShared::from(false);
        assert_eq!(bool_candy.to_string(), "true");
        assert_eq!(bool_candy_false.to_string(), "false");

        //Option
        let option_candy = CandyShared::from(Some(Box::from(CandyShared::from(123))));
        let option_candy_none = CandyShared::from(None);
        assert_eq!(option_candy.to_string(), "123");
        assert_eq!(option_candy_none.to_string(), "null");

        //Blob
        let blob_candy = CandyShared::from(vec![1_u8, 2_u8, 3_u8]);
        let blob_candy_2 = CandyShared::from(vec![15_u8]);
        assert_eq!(blob_candy.to_string(), "010203");
        assert_eq!(blob_candy_2.to_string(), "0f");

        //Bytes
        let bytes = Bytes(vec![1_u8, 2_u8, 3_u8]);
        let bytes_2 = Bytes(vec![15_u8]);
        assert_eq!(bytes.to_string(), "010203");
        assert_eq!(bytes_2.to_string(), "0f");

        //Array
        let array = CandyShared::from(vec![
            CandyShared::from(1),
            CandyShared::from("text"),
            CandyShared::from(3),
        ]);

        assert_eq!(array.to_string(), "[{1} {text} {3}]".to_string());

        //Nats
        let nats = CandyShared::from(vec![123_u128, 1234_u128, 12345_u128]);

        assert_eq!(nats.to_string(), "[123 1_234 12_345]".to_string());

        //Class
        let prop = vec![
            PropertyShared {
                name: "name".to_string(),
                value: CandyShared::from("some text".to_string()),
                immutable: false,
            },
            PropertyShared {
                name: "name_2".to_string(),
                value: CandyShared::from("another text".to_string()),
                immutable: true,
            },
        ];
        assert_eq!(
            CandyShared::from(prop).to_string(),
            "{name:some text; name_2:var another text;}".to_string()
        );

        //Principal
        let principal = Principal::anonymous();
        assert_eq!(CandyShared::from(principal).to_string(), "2vxsx-fae");

        //Floats
        let floats = vec![12.35, 25.66].to_candy();

        assert_eq!(floats.to_string(), "[12.35 25.66]".to_string());
    }

    #[test]
    fn conversion_to_blob() {
        // Nat
        let num = 255_u128;
        assert_eq!(num.to_candy().to_blob(), vec![255]);

        // Nat8
        let num = 255_u8;
        assert_eq!(CandyShared::from(num).to_blob(), vec![255]);

        // Nat16
        let num = 2566_u16;
        assert_eq!(CandyShared::from(num).to_blob(), vec![10, 6]);

        // Nat32
        let num = 255_u32;
        assert_eq!(CandyShared::from(num).to_blob(), vec![0, 0, 0, 255]);

        //Nat64
        let num = 300_000_u64;
        assert_eq!(
            CandyShared::from(num).to_blob(),
            vec![0, 0, 0, 0, 0, 4, 147, 224]
        );

        //Int
        let num = -123_i128;
        assert_eq!(CandyShared::from(num).to_blob(), vec![1, 123]);

        // Text
        let text = "Hello, world!".to_string();
        assert_eq!(
            CandyShared::from(text).to_blob(),
            vec![
                0, 0, 0, 72, 0, 0, 0, 101, 0, 0, 0, 108, 0, 0, 0, 108, 0, 0, 0, 111, 0, 0, 0, 44,
                0, 0, 0, 32, 0, 0, 0, 119, 0, 0, 0, 111, 0, 0, 0, 114, 0, 0, 0, 108, 0, 0, 0, 100,
                0, 0, 0, 33,
            ]
        );

        //Bytes
        let bytes = Bytes(vec![1_u8, 2_u8, 3_u8]);
        assert_eq!(bytes.to_blob(), vec![1_u8, 2_u8, 3_u8]);

        //Principal
        let principal = Principal::anonymous();
        let result = CandyShared::from(principal).to_blob();
        assert_eq!(result, vec![4]);
    }

    #[test]
    fn conversion_to_json() {
        // Nat
        assert_eq!(CandyShared::from(123_u128).to_json(), "123");
        assert_eq!(CandyShared::from(123_u64).to_json(), "123");
        assert_eq!(CandyShared::from(123_u64).to_json(), "123");
        assert_eq!(CandyShared::from(123_u32).to_json(), "123");
        assert_eq!(CandyShared::from(123_u16).to_json(), "123");
        assert_eq!(CandyShared::from(123_u8).to_json(), "123");
        assert_eq!(CandyShared::from("text").to_json(), "\"text\"");
        assert_eq!(CandyShared::from("text").to_json(), "\"text\"");
        assert_eq!(
            CandyShared::from(vec![
                PropertyShared {
                    value: CandyShared::from(123_u8),
                    name: "test".to_string(),
                    immutable: true
                },
                PropertyShared {
                    value: CandyShared::from(124_u8),
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
        assert_eq!(CandyShared::from(None).to_json(), "null");

        assert_eq!(
            Principal::management_canister().to_candy().to_json(),
            "\"aaaaa-aa\""
        );

        let nats = vec![123_u128, 1234_u128, 12345_u128].to_candy();
        assert_eq!(nats.to_json(), "[123,1_234,12_345]");

        //Floats
        let floats = vec![12.35, 25.66].to_candy();
        assert_eq!(
            vec![1.0, 2.0, 3.5, 4.123].to_candy().to_json(),
            "[1,2,3.5,4.123]"
        );
        assert_eq!(floats.to_json(), "[12.35,25.66]");

        //Bytes
        let bytes = Bytes(vec![1_u8, 2_u8, 3_u8]);
        let bytes_2 = Bytes(vec![15_u8]);
        assert_eq!(bytes.to_json(), "\"010203\"");
        assert_eq!(bytes_2.to_json(), "\"0f\"");

        //Blob
        let blob = CandyShared::from(vec![1_u8, 2_u8, 3_u8]);
        assert_eq!(blob.to_json(), "\"010203\"");

        //Principal
        let principal = Principal::anonymous();
        let result = CandyShared::from(principal).to_json();
        assert_eq!(result, "\"2vxsx-fae\"");

        //Bool
        let t = true.to_candy();
        let f = false.to_candy();
        assert_eq!(t.to_json(), "\"true\"");
        assert_eq!(f.to_json(), "\"false\"");

        //Float
        let f = 12.35.to_candy();
        assert_eq!(f.to_json(), "12.35");
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
