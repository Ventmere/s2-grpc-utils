# S2 gRPC Utilities

Utilities for gRPC/Proto Buffers.

## Usage

See `s2-job-queue` project.

## Libraries

- Protocol Buffers implementation: [prost](https://github.com/danburkert/prost)
- gRPC implementation: [tonic](https://github.com/hyperium/tonic)

## Type Conventions

All `Protobuf Type`s that have `google.protobuf` namespace are [Protocol Buffers Well-Known Types](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf). We use [prost-types](https://docs.rs/prost-types/0.5.0/prost_types/) as their Rust representation. Users should not need to interact with types from `prost-types` directly.

### JSON value

| Rust Type                                                             | Protobuf Type           |
| --------------------------------------------------------------------- | ----------------------- |
| [serde_json::Value](https://docs.serde.rs/serde_json/enum.Value.html) | `google.protobuf.Value` |

### Timestamp

| Rust Type                                                                               | Protobuf Type               |
| --------------------------------------------------------------------------------------- | --------------------------- |
| [chrono::DateTime&lt;Utc&gt;](https://docs.rs/chrono/0.4.9/chrono/struct.DateTime.html) | `google.protobuf.Timestamp` |

### Optional/Nullable Types

In `proto3`, all fields are "optional" (in that it is not an error if the sender fails to set them). But, fields are no longer "nullable", in that there's no way to tell the difference between a field being explicitly set to its default value vs. not having been set at all.

To represent a Rust `Option<T>`, we use [Wrappers](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/wrappers.proto).

For scalar types:

| Rust Type        | Protobuf Type                 |
| ---------------- | ----------------------------- |
| `Option<f32>`    | `google.protobuf.FloatValue`  |
| `Option<f64>`    | `google.protobuf.DoubleValue` |
| `Option<i64>`    | `google.protobuf.Int64Value`  |
| `Option<u64>`    | `google.protobuf.UInt64Value` |
| `Option<i32>`    | `google.protobuf.Int32Value`  |
| `Option<u32>`    | `google.protobuf.UInt32Value` |
| `Option<bool>`   | `google.protobuf.BoolValue`   |
| `Option<String>` | `google.protobuf.StringValue` |

We don't need special treatment for complex types (structs) because they are always wrapped by `Option<...>`. There is no way to define a non-optional complex field in `proto3`.

### Enumerations

Enumeration variants must have a disriminant.

```rust
use prost::Enumeration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enumeration)]
pub enum Gender {
  Unknown = 0,
  Female = 1,
  Male = 2,
}
```
