use prost_types::value::Kind;
use prost_types::{Struct, Value};
use s2_grpc_utils::{S2ProtoEnum, S2ProtoPack, S2ProtoUnpack};
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Message {
  v1: i32,
  v2: String,
  json: Option<Value>,
  json_optional: Option<Value>,
  elements: Vec<NestedMessage>,
  map: HashMap<i32, NestedMessage>,
}

#[derive(Debug, PartialEq, Clone)]
struct NestedMessage {
  v: i32,
}

fn map_i32(v: i32) -> i32 {
  v
}

#[derive(Debug, S2ProtoPack, S2ProtoUnpack, PartialEq)]
#[s2_grpc(message_type = "Message")]
struct Model {
  #[s2_grpc(map_fn = "map_i32")]
  v1: i32,
  v2: String,
  json: JsonValue,
  json_optional: Option<JsonValue>,
  elements: Vec<NestedModel>,
  map: HashMap<i32, NestedModel>,
}

#[derive(Debug, S2ProtoPack, S2ProtoUnpack, PartialEq)]
#[s2_grpc(message_type = "NestedMessage")]
struct NestedModel {
  v: i32,
}

#[test]
fn derive_enum() {
  use crate::S2ProtoEnum;

  #[derive(Debug, PartialEq)]
  enum EnumProto {
    A = 0,
    B = 1,
  }

  impl EnumProto {
    fn from_i32(v: i32) -> Option<Self> {
      match v {
        0 => Some(EnumProto::A),
        1 => Some(EnumProto::B),
        _ => None,
      }
    }
  }

  #[derive(Debug, S2ProtoEnum, PartialEq)]
  #[s2_grpc(proto_enum_type = "EnumProto")]
  enum EnumModel {
    A,
    B,
  }

  assert_eq!(EnumModel::from_i32(1), Some(EnumModel::B));
  assert_eq!(EnumModel::B.pack(), EnumProto::B);
  assert_eq!(EnumModel::B.get_variant_name(), "B");
  assert_eq!(EnumModel::NAME, "EnumModel");
}

#[test]
fn derive() {
  let mut map = HashMap::new();
  map.insert(1, NestedMessage { v: 2 });

  let msg = Message {
    v1: 1,
    v2: "text".to_string(),
    json: Some(Value {
      kind: Some(Kind::StructValue(Struct {
        fields: vec![(
          "v".to_string(),
          Value {
            kind: Some(Kind::NumberValue(1_f64)),
          },
        )]
        .into_iter()
        .collect(),
      })),
    }),
    json_optional: None,
    elements: vec![NestedMessage { v: 111 }],
    map,
  };

  let model = Model::unpack(msg.clone()).unwrap();

  let mut map = HashMap::new();
  map.insert(1, NestedModel { v: 2 });

  assert_eq!(
    model,
    Model {
      v1: 1,
      v2: "text".to_string(),
      json: json!({
        "v": 1_f64
      }),
      json_optional: None,
      elements: vec![NestedModel { v: 111 }],
      map
    }
  );

  let msg_: Message = model.pack().unwrap();
  assert_eq!(msg_, msg);
}

#[test]
fn derive_err() {
  let msg = Message {
    v1: 1,
    v2: "text".to_string(),
    json: None,
    json_optional: None,
    elements: vec![],
    map: HashMap::new(),
  };

  let err = Model::unpack(msg.clone()).err().unwrap();
  assert_eq!(
    format!("{}", err),
    "Could not unpack field 'json' from null"
  )
}
