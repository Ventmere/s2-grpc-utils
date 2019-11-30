use prost_types::value::Kind;
use prost_types::{Struct, Value};
use s2_grpc_utils::{S2ProtoPack, S2ProtoUnpack};
use serde_json::{json, Value as JsonValue};

#[derive(Debug, PartialEq, Clone)]
struct Message {
  v1: i32,
  v2: String,
  json: Option<Value>,
  json_optional: Option<Value>,
}

#[derive(Debug, S2ProtoPack, S2ProtoUnpack, PartialEq)]
#[s2_grpc(message_type = "Message")]
struct Model {
  v1: i32,
  v2: String,
  json: JsonValue,
  json_optional: Option<JsonValue>,
}

#[test]
fn derive() {
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
  };

  let model = Model::unpack(msg.clone()).unwrap();

  assert_eq!(
    model,
    Model {
      v1: 1,
      v2: "text".to_string(),
      json: json!({
        "v": 1_f64
      }),
      json_optional: None,
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
  };

  let err = Model::unpack(msg.clone()).err().unwrap();
  assert_eq!(
    format!("{}", err),
    "Could not unpack field 'json' from null"
  )
}
