use chrono::{DateTime, Utc};
use prost_types;
use prost_types::{Timestamp, Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use snafu::ResultExt;

use crate::result::{self, Result};
use crate::{S2ProtoPack, S2ProtoUnpack};

macro_rules! impl_option {
  ($rust:ty => $proto:ty) => {
    impl S2ProtoPack<Option<$proto>> for $rust {
      fn pack(self) -> Result<Option<$proto>> {
        Ok(Some(self.pack()?))
      }
    }

    impl S2ProtoUnpack<Option<$proto>> for $rust {
      fn unpack(value: Option<$proto>) -> Result<$rust> {
        if let Some(value) = value {
          Ok(<$rust>::unpack(value)?)
        } else {
          Err(result::Error::ValueNotPresent)
        }
      }
    }
  };
}

// JSON value

impl S2ProtoPack<Value> for JsonValue {
  fn pack(self) -> Result<Value> {
    json_value_to_value(self)
  }
}

impl S2ProtoUnpack<Value> for JsonValue {
  fn unpack(value: Value) -> Result<JsonValue> {
    value_to_json_value(value)
  }
}

impl_option!(JsonValue => Value);

const MAX_JSON_NEST: usize = 100;

fn value_to_json_value(value: Value) -> Result<JsonValue> {
  fn convert(nest: usize, value: Value) -> Result<JsonValue> {
    use prost_types::{value::Kind, ListValue, Struct};
    use serde_json::{Map as JsonMap, Number as JsonNumber};

    if nest >= MAX_JSON_NEST {
      return Err(result::Error::JsonValueNestedTooDeeply);
    }

    if let Some(kind) = value.kind {
      let converted = match kind {
        Kind::NullValue(_) => JsonValue::Null,
        Kind::NumberValue(v) => {
          if let Some(number) = JsonNumber::from_f64(v) {
            JsonValue::Number(number)
          } else {
            JsonValue::Null
          }
        }
        Kind::StringValue(v) => JsonValue::String(v),
        Kind::BoolValue(v) => JsonValue::Bool(v),
        Kind::StructValue(Struct { fields }) => JsonValue::Object({
          let mut json_map = JsonMap::with_capacity(fields.len());
          for (k, v) in fields {
            json_map.insert(k, convert(nest + 1, v)?);
          }
          json_map
        }),
        Kind::ListValue(ListValue { values }) => {
          let mut json_values = Vec::with_capacity(values.len());
          for v in values {
            json_values.push(convert(nest + 1, v)?);
          }
          JsonValue::Array(json_values)
        }
      };
      Ok(converted)
    } else {
      Ok(JsonValue::Null)
    }
  }

  convert(0, value)
}

fn json_value_to_value(value: JsonValue) -> Result<Value> {
  fn convert(nest: usize, value: JsonValue) -> Result<Value> {
    use prost_types::{value::Kind, ListValue, Struct};
    use std::collections::BTreeMap;

    if nest >= MAX_JSON_NEST {
      return Err(result::Error::JsonValueNestedTooDeeply);
    }

    let kind = match value {
      JsonValue::Null => Kind::NullValue(0),
      JsonValue::Bool(v) => Kind::BoolValue(v),
      JsonValue::Number(v) => {
        if let Some(v) = v.as_f64() {
          Kind::NumberValue(v)
        } else {
          Kind::NullValue(0)
        }
      }
      JsonValue::String(v) => Kind::StringValue(v),
      JsonValue::Array(values) => {
        let mut value_values = Vec::with_capacity(values.len());
        for v in values {
          value_values.push(convert(nest + 1, v)?);
        }
        Kind::ListValue(ListValue {
          values: value_values,
        })
      }
      JsonValue::Object(map) => {
        let mut value_map = BTreeMap::new();
        for (k, v) in map {
          value_map.insert(k, convert(nest + 1, v)?);
        }
        Kind::StructValue(Struct { fields: value_map })
      }
    };
    Ok(Value { kind: Some(kind) })
  }

  convert(0, value)
}

/// Helper type to convert any serializable type from/to `google.protobuf.Value`
pub struct Json<T>(pub T);

impl<T> S2ProtoPack<Value> for Json<T>
where
  T: Serialize + for<'de> Deserialize<'de>,
{
  fn pack(self) -> Result<Value> {
    pack_value(self.0)
  }
}

impl<T> S2ProtoUnpack<Value> for Json<T>
where
  T: Serialize + for<'de> Deserialize<'de>,
{
  fn unpack(value: Value) -> Result<Json<T>> {
    unpack_value(value).map(Json)
  }
}

pub fn pack_value<T>(value: T) -> Result<Value>
where
  T: Serialize,
{
  serde_json::to_value(&value).context(result::Json)?.pack()
}

pub fn unpack_value<T>(value: Value) -> Result<T>
where
  T: for<'de> Deserialize<'de>,
{
  let value = JsonValue::unpack(value)?;
  Ok(serde_json::from_value(value).context(result::Json)?)
}

// Timestamp

impl S2ProtoPack<Timestamp> for DateTime<Utc> {
  fn pack(self) -> Result<Timestamp> {
    Ok(Timestamp {
      seconds: self.timestamp(),
      nanos: self.timestamp_subsec_nanos() as i32,
    })
  }
}

impl S2ProtoUnpack<Timestamp> for DateTime<Utc> {
  fn unpack(Timestamp { seconds, nanos }: Timestamp) -> Result<DateTime<Utc>> {
    let dt = chrono::NaiveDateTime::from_timestamp(seconds, nanos as u32);
    Ok(DateTime::from_utc(dt, Utc))
  }
}

impl_option!(DateTime<Utc> => Timestamp);

// Wrappers

macro_rules! impl_self {
  (
    $($ty:ty),*
  ) => {
    $(
      impl S2ProtoPack<$ty> for $ty {
        fn pack(self) -> Result<$ty> {
          Ok(self)
        }
      }

      impl S2ProtoUnpack<$ty> for $ty {
        fn unpack(value: $ty) -> Result<$ty> {
          Ok(value)
        }
      }
    )*
  }
}

impl_self! {
  f32,
  f64,
  i64,
  u64,
  i32,
  u32,
  bool,
  String
}
