#![allow(unused_imports)]

#[cfg(test)]
mod collections_test {
    use std::collections::{HashMap, HashSet};

    use candid::Principal;

    use candy::types::PropertyShared;
    use candy::value::{CandyShared, ToCandyValue};

    use super::*;

    #[test]
    fn hash_map() {
        let mut map: HashMap<CandyShared, CandyShared> = HashMap::new();
        map.insert(1_u128.to_candy(), 2_u128.to_candy());
        map.insert(3_u8.to_candy(), 4_u8.to_candy());
        map.insert(5_u16.to_candy(), 6_u16.to_candy());
        map.insert(7_u32.to_candy(), 8_u32.to_candy());
        map.insert(9_u64.to_candy(), 10_u64.to_candy());
        map.insert(11_i128.to_candy(), 12_i128.to_candy());
        map.insert(13_i8.to_candy(), 14_i8.to_candy());
        map.insert(15_i16.to_candy(), 16_i16.to_candy());
        map.insert(17_i32.to_candy(), 18_i32.to_candy());
        map.insert(19_i64.to_candy(), 20_i64.to_candy());
        map.insert("key".to_candy(), "value".to_candy());
        map.insert(false.to_candy(), true.to_candy());
        map.insert(
            vec![0_u8, 1_u8, 2_u8, 3_u8, 4_u8].to_candy(),
            vec![4_u8, 5_u8, 6_u8, 7_u8, 8_u8].to_candy(),
        );
        map.insert(
            CandyShared::Bytes(vec![0_u8, 1_u8, 2_u8, 3_u8, 4_u8]),
            CandyShared::Bytes(vec![0_u8, 1_u8, 2_u8, 3_u8, 4_u8]),
        );
        map.insert(
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
        map.insert(
            Principal::anonymous().to_candy(),
            Principal::management_canister().to_candy(),
        );
        map.insert(Some(Box::from(1.to_candy())).to_candy(), None.to_candy());
        map.insert(
            vec![1.to_candy(), 2.to_candy()].to_candy(),
            vec![3.to_candy(), 4.to_candy()].to_candy(),
        );
        map.insert(
            vec![1_u128.to_candy(), 2_u128.to_candy()].to_candy(),
            vec![3_u128.to_candy(), 4_u128.to_candy()].to_candy(),
        );
        map.insert(
            vec![1_f64.to_candy(), 2_f64.to_candy()].to_candy(),
            vec![3_f64.to_candy(), 4_f64.to_candy()].to_candy(),
        );

        // Nat
        let key = 1_u128.to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&2_u128.to_candy()));

        // Nat8
        let key = 3_u8.to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&4_u8.to_candy()));

        // Nat16
        let key = 5_u16.to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&6_u16.to_candy()));

        // Nat32
        let key = 7_u32.to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&8_u32.to_candy()));

        // Nat64
        let key = 9_u64.to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&10_u64.to_candy()));

        // Int
        let key = 11_i128.to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&12_i128.to_candy()));

        // Int8
        let key = 13_i8.to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&14_i8.to_candy()));

        // Int16
        let key = 15_i16.to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&16_i16.to_candy()));

        // Int32
        let key = 17_i32.to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&18_i32.to_candy()));

        // Int64
        let key = 19_i64.to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&20_i64.to_candy()));

        // Text
        let key = "key".to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&"value".to_candy()));

        // Bool
        let key = false.to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&true.to_candy()));

        // Blob
        let key = vec![0_u8, 1_u8, 2_u8, 3_u8, 4_u8].to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&vec![4_u8, 5_u8, 6_u8, 7_u8, 8_u8].to_candy()));

        // Bytes
        let key = CandyShared::Bytes(vec![0_u8, 1_u8, 2_u8, 3_u8, 4_u8]);
        let res = map.get(&key);
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
        let res = map.get(&key);
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
        let res = map.get(&key);
        assert_eq!(res, Some(&Principal::management_canister().to_candy()));

        //Option
        let key = Some(Box::from(1.to_candy())).to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&None.to_candy()));

        // Array
        let key = vec![1.to_candy(), 2.to_candy()].to_candy();
        let res = map.get(&key);
        assert_eq!(res, Some(&vec![3.to_candy(), 4.to_candy()].to_candy()));

        // Nats
        let key = vec![1_u128.to_candy(), 2_u128.to_candy()].to_candy();
        let res = map.get(&key);
        assert_eq!(
            res,
            Some(&vec![3_u128.to_candy(), 4_u128.to_candy()].to_candy())
        );

        // Floats
        let key = vec![1_f64.to_candy(), 2_f64.to_candy()].to_candy();
        let res = map.get(&key);
        assert_eq!(
            res,
            Some(&vec![3_f64.to_candy(), 4_f64.to_candy()].to_candy())
        );

        println!("{:?}", res);
        println!("{:?}", map.to_candy());
    }
}
