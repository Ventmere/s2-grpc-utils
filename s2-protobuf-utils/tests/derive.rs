use prost_types::Any;
use s2_protobuf_utils::S2Proto;
use serde_json::{json, Value};

#[derive(Debug, PartialEq, Clone)]
struct Message {
  v1: i32,
  v2: String,
  json: Any,
  json_optional: Option<Any>,
}

#[derive(Debug, S2Proto, PartialEq)]
#[s2_proto(message_type = "Message")]
struct Model {
  v1: i32,
  v2: String,
  json: Value,
  json_optional: Option<Value>,
}

#[test]
fn derive_s2_proto() {
  let msg = Message {
    v1: 1,
    v2: "text".to_string(),
    json: Any {
      type_url: "s2/json".to_string(),
      value: br#"{"v":1}"#.to_vec(),
    },
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
