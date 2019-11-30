use prost_types::Any;
use s2_grpc_utils::{S2ProtoPack, S2ProtoUnpack};
use serde_json::{json, Value};

#[derive(Debug, PartialEq, Clone)]
struct Message {
  v1: i32,
  v2: String,
  json: Option<Any>,
  json_optional: Option<Any>,
}

#[derive(Debug, S2ProtoPack, S2ProtoUnpack, PartialEq)]
#[s2_grpc(message_type = "Message")]
struct Model {
  v1: i32,
  v2: String,
  json: Value,
  json_optional: Option<Value>,
}

#[test]
fn derive() {
  let msg = Message {
    v1: 1,
    v2: "text".to_string(),
    json: Some(Any {
      type_url: "s2/json".to_string(),
      value: br#"{"v":1}"#.to_vec(),
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
        "v": 1
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
