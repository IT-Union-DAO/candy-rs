use std::string::ToString;


use candid::Principal;
use crate::stable::value::CandyValue;
use num_traits::cast::ToPrimitive;

pub trait UnboxCandyValue {
    fn to_nat(self) -> Option<u128>;
    fn to_nat8(self) -> Option<u8>;
    fn to_nat16(self) -> u16;
    fn to_nat32(self) -> u32;
    fn to_nat64(self) -> u64;
    fn to_int(self) -> i128;
    fn to_int8(self) -> i8;
    fn to_int16(self) -> i16;
    fn to_int32(self) -> i32;
    fn to_int64(self) -> i64;
    fn to_float(self) -> f64;
    fn to_text(self) -> String;
    fn to_principal(self) -> Principal;
}

trait UnboxCandyValueUnstable: UnboxCandyValue {
    fn to_value_array(self) -> Box<[CandyValue]>;
    fn to_blob(self) -> Box<[u8]>;
    fn to_bytes(self) -> Box<[u8]>;
    fn to_bytes_buffer(self) -> Vec<u8>;
    fn to_floats_buffer(self) -> Vec<u8>;
}


impl UnboxCandyValue for CandyValue {

    fn to_nat(self) -> Option<u128> {
        match self {
            Self::Nat(val) => Some(val),
            Self::Nat8(val) => val.to_u128(),
            Self::Nat16(val) => val.to_u128(),
            Self::Nat32(val) => val.to_u128(),
            Self::Nat64(val) => val.to_u128(),
            Self::Float(val) => val.round().to_u128(),
            Self::Int(val) => val.to_u128(),
            Self::Int8(val) => val.to_u128(),
            Self::Int16(val) => val.to_u128(),
            Self::Int32(val) => val.to_u128(),
            Self::Int64(val) => val.to_u128(),
            _ => None
        }
    }

    fn to_nat8(self) -> Option<u8> {
        todo!()
    }

    fn to_nat16(self) -> u16 {
        todo!()
    }

    fn to_nat32(self) -> u32 {
        todo!()
    }

    fn to_nat64(self) -> u64 {
        todo!()
    }

    fn to_int(self) -> i128 {
        todo!()
    }

    fn to_int8(self) -> i8 {
        todo!()
    }

    fn to_int16(self) -> i16 {
        todo!()
    }

    fn to_int32(self) -> i32 {
        todo!()
    }

    fn to_int64(self) -> i64 {
        todo!()
    }

    fn to_float(self) -> f64 {
        todo!()
    }

    fn to_text(self) -> String {
        todo!()
    }

    fn to_principal(self) -> Principal {
        todo!()
    }
}