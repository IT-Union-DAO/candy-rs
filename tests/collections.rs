#![allow(unused_imports)]

#[cfg(test)]
mod collections_test {
    use std::collections::HashMap;

    use candid::Principal;

    use ic_candy::types::PropertyShared;
    use ic_candy::value::{CandyShared, ToCandyValue};

    #[test]
    fn hash_map() {
        let mut valueMap: HashMap<CandyShared, CandyShared> = HashMap::new();
        valueMap.insert(1_u128.to_candy(), 2_u128.to_candy());
        valueMap.insert(3_u8.to_candy(), 4_u8.to_candy());
        valueMap.insert(5_u16.to_candy(), 6_u16.to_candy());
        valueMap.insert(7_u32.to_candy(), 8_u32.to_candy());
        valueMap.insert(9_u64.to_candy(), 10_u64.to_candy());
        valueMap.insert(11_i128.to_candy(), 12_i128.to_candy());
        valueMap.insert(13_i8.to_candy(), 14_i8.to_candy());
        valueMap.insert(15_i16.to_candy(), 16_i16.to_candy());
        valueMap.insert(17_i32.to_candy(), 18_i32.to_candy());
        valueMap.insert(19_i64.to_candy(), 20_i64.to_candy());
        valueMap.insert("key".to_candy(), "value".to_candy());
        valueMap.insert(false.to_candy(), true.to_candy());
        valueMap.insert(
            vec![0_u8, 1_u8, 2_u8, 3_u8, 4_u8].to_candy(),
            vec![4_u8, 5_u8, 6_u8, 7_u8, 8_u8].to_candy(),
        );
        valueMap.insert(
            CandyShared::Bytes(vec![0_u8, 1_u8, 2_u8, 3_u8, 4_u8]),
            CandyShared::Bytes(vec![0_u8, 1_u8, 2_u8, 3_u8, 4_u8]),
        );
        valueMap.insert(
            vec![PropertyShared {
                name: "key".to_string(),
                immutable: false,
                value: "value".to_candy(),
            }]
                .to_candy(),
            vec![PropertyShared {
                name: "key2".to_string(),
                immutable: false,
                value: "value2".to_candy(),
            }]
                .to_candy(),
        );
        valueMap.insert(
            Principal::anonymous().to_candy(),
            Principal::management_canister().to_candy(),
        );
        valueMap.insert(Some(Box::from(1.to_candy())).to_candy(), None.to_candy());
        valueMap.insert(
            vec![1.to_candy(), 2.to_candy()].to_candy(),
            vec![3.to_candy(), 4.to_candy()].to_candy(),
        );
        valueMap.insert(
            vec![1_u128.to_candy(), 2_u128.to_candy()].to_candy(),
            vec![3_u128.to_candy(), 4_u128.to_candy()].to_candy(),
        );
        valueMap.insert(
            vec![1_f64.to_candy(), 2_f64.to_candy()].to_candy(),
            vec![3_f64.to_candy(), 4_f64.to_candy()].to_candy(),
        );

        // Nat
        let key = 1_u128.to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&2_u128.to_candy()));

        // Nat8
        let key = 3_u8.to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&4_u8.to_candy()));

        // Nat16
        let key = 5_u16.to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&6_u16.to_candy()));

        // Nat32
        let key = 7_u32.to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&8_u32.to_candy()));

        // Nat64
        let key = 9_u64.to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&10_u64.to_candy()));

        // Int
        let key = 11_i128.to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&12_i128.to_candy()));

        // Int8
        let key = 13_i8.to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&14_i8.to_candy()));

        // Int16
        let key = 15_i16.to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&16_i16.to_candy()));

        // Int32
        let key = 17_i32.to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&18_i32.to_candy()));

        // Int64
        let key = 19_i64.to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&20_i64.to_candy()));

        // Text
        let key = "key".to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&"value".to_candy()));

        // Bool
        let key = false.to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&true.to_candy()));

        // Blob
        let key = vec![0_u8, 1_u8, 2_u8, 3_u8, 4_u8].to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&vec![4_u8, 5_u8, 6_u8, 7_u8, 8_u8].to_candy()));

        // Bytes
        let key = CandyShared::Bytes(vec![0_u8, 1_u8, 2_u8, 3_u8, 4_u8]);
        let res = valueMap.get(&key);
        assert_eq!(
            res,
            Some(&CandyShared::Bytes(vec![0_u8, 1_u8, 2_u8, 3_u8, 4_u8]))
        );

        // Class
        let key = vec![PropertyShared {
            name: "key".to_string(),
            immutable: false,
            value: "value".to_candy(),
        }]
            .to_candy();
        let res = valueMap.get(&key);
        assert_eq!(
            res,
            Some(
                &vec![PropertyShared {
                    name: "key2".to_string(),
                    immutable: false,
                    value: "value2".to_candy(),
                }]
                    .to_candy()
            )
        );

        // Principal
        let key = Principal::anonymous().to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&Principal::management_canister().to_candy()));

        //Option
        let key = Some(Box::from(1.to_candy())).to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&None.to_candy()));

        // Array
        let key = vec![1.to_candy(), 2.to_candy()].to_candy();
        let res = valueMap.get(&key);
        assert_eq!(res, Some(&vec![3.to_candy(), 4.to_candy()].to_candy()));

        // Nats
        let key = vec![1_u128.to_candy(), 2_u128.to_candy()].to_candy();
        let res = valueMap.get(&key);
        assert_eq!(
            res,
            Some(&vec![3_u128.to_candy(), 4_u128.to_candy()].to_candy())
        );

        // Floats
        let key = vec![1_f64.to_candy(), 2_f64.to_candy()].to_candy();
        let res = valueMap.get(&key);
        assert_eq!(
            res,
            Some(&vec![3_f64.to_candy(), 4_f64.to_candy()].to_candy())
        );

        println!("{:?}", res);
        println!("{:?}", valueMap.to_candy());
    }

    #[test]
    fn candy_map() {
        let mut text_map = HashMap::<String, CandyShared>::new();
        text_map.insert("icp".to_string(), 123_u128.to_candy());

        let candy_map = text_map.to_candy();
        if let CandyShared::Map(map) = candy_map {
            assert_eq!(123_u128.to_candy(), *map.get("icp").unwrap());
        }
    }
}
