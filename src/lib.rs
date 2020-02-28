mod convert;
pub mod result;

use crate::result::Error;

#[allow(unused_imports)]
#[macro_use]
extern crate s2_grpc_utils_derive;

pub use self::convert::Json;
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

pub trait S2ProtoEnum
where
  Self: Sized,
{
  type ProtoEnum;
  const NAME: &'static str;

  fn from_i32(v: i32) -> Option<Self>;
  fn pack(&self) -> Self::ProtoEnum;
  fn get_variant_name(&self) -> &'static str;
}

impl<T1, T2> S2ProtoPack<Option<T1>> for Option<T2>
where
  T2: S2ProtoPack<T1>,
{
  fn pack(self) -> Result<Option<T1>, Error> {
    if let Some(value) = self {
      Ok(Some(value.pack()?))
    } else {
      Ok(None)
    }
  }
}

impl<T1, T2> S2ProtoUnpack<Option<T1>> for Option<T2>
where
  T2: S2ProtoUnpack<T1>,
{
  fn unpack(value: Option<T1>) -> Result<Self, Error> {
    if let Some(value) = value {
      Ok(Some(T2::unpack(value)?))
    } else {
      Ok(None)
    }
  }
}
