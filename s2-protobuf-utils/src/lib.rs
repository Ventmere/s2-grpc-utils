/// Converts a Rust value to its protobuf form.
pub trait IntoProto {
  type Target;
  fn into_proto(self) -> Self::Target;
}
