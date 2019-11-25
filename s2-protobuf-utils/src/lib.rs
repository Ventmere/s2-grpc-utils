mod convert;
pub mod result;

use crate::result::Error;

#[allow(unused_imports)]
#[macro_use]
extern crate s2_protobuf_utils_derive;

pub use s2_protobuf_utils_derive::*;

pub trait S2Proto<T>
where
  Self: Sized,
{
  fn pack(self) -> Result<T, Error>;
  fn unpack(value: T) -> Result<Self, Error>;
}
