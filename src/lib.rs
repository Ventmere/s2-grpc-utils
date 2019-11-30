mod convert;
pub mod result;

use crate::result::Error;

#[allow(unused_imports)]
#[macro_use]
extern crate s2_grpc_utils_derive;

pub use self::convert::{pack_any, unpack_any, Json};
pub use s2_grpc_utils_derive::*;

pub trait S2ProtoPack<T>
where
  Self: Sized,
{
  fn pack(self) -> Result<T, Error>;
}

pub trait S2ProtoUnpack<T>
where
  Self: Sized,
{
  fn unpack(value: T) -> Result<Self, Error>;
}
