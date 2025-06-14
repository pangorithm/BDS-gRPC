use prost_types::Struct;
use prost_types::Value as ProstValue;
use prost_types::value::Kind;
use serde_json::{Map, Value as JsonValue};

pub fn struct_to_value(s: Struct) -> JsonValue {
    let mut map = Map::new();
    for (k, v) in s.fields {
        map.insert(k, prost_value_to_json(v));
    }
    JsonValue::Object(map)
}

pub fn prost_value_to_json(value: ProstValue) -> JsonValue {
    match value.kind {
        Some(Kind::NullValue(_)) => JsonValue::Null,
        Some(Kind::NumberValue(n)) => JsonValue::Number(
            serde_json::Number::from_f64(n).unwrap_or(serde_json::Number::from(0)),
        ),
        Some(Kind::StringValue(s)) => JsonValue::String(s),
        Some(Kind::BoolValue(b)) => JsonValue::Bool(b),
        Some(Kind::StructValue(s)) => struct_to_value(s),
        Some(Kind::ListValue(l)) => {
            JsonValue::Array(l.values.into_iter().map(prost_value_to_json).collect())
        }
        None => JsonValue::Null,
    }
}
