extern crate proc_macro;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod derive_s2_proto;

#[proc_macro_derive(S2Proto, attributes(s2_proto))]
pub fn try_from_proto(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let receiver = derive_s2_proto::InputReceiver::from_derive_input(&input).unwrap();
  // panic!("{}", quote!(#receiver));
  TokenStream::from(quote!(#receiver))
}
