use crate::stable::types::Property;
use crate::stable::values::CandyValue;
use crate::unstable::types::{Array, Floats};
use crate::unstable::types::Bytes;
use crate::unstable::types::Nats;
use crate::unstable::types::PropertyUnstable;
use crate::unstable::values::CandyValueUnstable;

type StableArray = crate::stable::types::Array;
type StableBytes = crate::stable::types::Bytes;
type StableFloats = crate::stable::types::Floats;
type StableNats = crate::stable::types::Nats;

pub fn stabilize_value(item: CandyValueUnstable) -> CandyValue {
    match item {
        CandyValueUnstable::Int(val) => CandyValue::Int(val),
        CandyValueUnstable::Int8(val) => CandyValue::Int8(val),
        CandyValueUnstable::Int16(val) => CandyValue::Int16(val),
        CandyValueUnstable::Int32(val) => CandyValue::Int32(val),
        CandyValueUnstable::Int64(val) => CandyValue::Int64(val),
        CandyValueUnstable::Nat(val) => CandyValue::Nat(val),
        CandyValueUnstable::Nat8(val) => CandyValue::Nat8(val),
        CandyValueUnstable::Nat16(val) => CandyValue::Nat16(val),
        CandyValueUnstable::Nat32(val) => CandyValue::Nat32(val),
        CandyValueUnstable::Nat64(val) => CandyValue::Nat64(val),
        CandyValueUnstable::Float(val) => CandyValue::Float(val),
        CandyValueUnstable::Text(val) => CandyValue::Text(val),
        CandyValueUnstable::Bool(val) => CandyValue::Bool(val),
        CandyValueUnstable::Blob(val) => CandyValue::Blob(val),
        CandyValueUnstable::Class(val) => {
            let stabalized_props = val.iter()
                .map(|prop| stabilize_property(prop.clone()))
                .collect::<Vec<Property>>();
            CandyValue::Class(Box::from(stabalized_props))
        }
        CandyValueUnstable::Principal(val) => CandyValue::Principal(val),
        CandyValueUnstable::Array(val) => match val {
            Array::Frozen(val) =>
                CandyValue::Array(
                    StableArray::Frozen(Box::from(stabilize_value_array(val)))
                ),
            Array::Thawed(val) =>
                CandyValue::Array(
                    StableArray::Thawed(
                        Box::from(stabilize_value_array(Box::from(val)))
                    ))
        },
        CandyValueUnstable::Option(val) => match val {
            Some(val) => CandyValue::Option(Some(Box::from(stabilize_value(*val)))),
            None => CandyValue::Option(None)
        },
        CandyValueUnstable::Bytes(val) => match val {
            Bytes::Frozen(val) => CandyValue::Bytes(StableBytes::Frozen(val)),
            Bytes::Thawed(val) => CandyValue::Bytes(StableBytes::Thawed(Box::from(val)))
        },
        CandyValueUnstable::Floats(val) => match val {
            Floats::Frozen(val) => CandyValue::Floats(
                StableFloats::Frozen(val)),
            Floats::Thawed(val) => CandyValue::Floats(
                StableFloats::Thawed(val.as_slice().into()))
        },
        CandyValueUnstable::Nats(val) => match val {
            Nats::Frozen(val) => CandyValue::Nats(StableNats::Frozen(val.clone())),
            Nats::Thawed(val) => CandyValue::Nats(
                StableNats::Thawed(val.into())
            )
        }
        CandyValueUnstable::Empty => CandyValue::Empty
    }
}

pub fn destabilize_value(item: CandyValue) -> CandyValueUnstable {
    match item {
        CandyValue::Int(val) => CandyValueUnstable::Int(val),
        _ => CandyValueUnstable::Empty
    }
}

pub fn destabilize_property(item: Property) -> PropertyUnstable {
    PropertyUnstable {
        name: item.name,
        value: destabilize_value(item.value),
        immutable: item.immutable,
    }
}

pub fn stabilize_property(item: PropertyUnstable) -> Property {
    Property {
        name: item.name,
        value: stabilize_value(item.value),
        immutable: item.immutable,
    }
}

pub fn stabilize_value_array(items: Box<[CandyValueUnstable]>) -> Vec<CandyValue> {
    let mut stabilized_values = vec![];
    for (_, item) in items.iter().enumerate() {
        stabilized_values.push(stabilize_value(item.clone()));
    }
    stabilized_values
}



