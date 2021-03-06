extern crate proc_macro;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod derive_s2_proto;
mod derive_s2_proto_enum;

macro_rules! try_parse {
  ($e:expr) => {
    match $e {
      Ok(v) => v,
      Err(e) => return TokenStream::from(e.write_errors()),
    }
  };
}

#[proc_macro_derive(S2ProtoPack, attributes(s2_grpc))]
pub fn derive_pack(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let receiver = try_parse!(derive_s2_proto::InputReceiver::from_derive_input(&input));
  TokenStream::from(quote!(#receiver))
}

#[proc_macro_derive(S2ProtoUnpack, attributes(s2_grpc))]
pub fn derive_unpack(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let receiver = try_parse!(derive_s2_proto::InputReceiver::from_derive_input(&input)).to_unpack();
  TokenStream::from(quote!(#receiver))
}

#[proc_macro_derive(S2ProtoEnum, attributes(s2_grpc))]
pub fn derive_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let receiver = try_parse!(derive_s2_proto_enum::InputReceiver::from_derive_input(
    &input
  ));
  TokenStream::from(quote!(#receiver))
}
