use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::Paths;

#[derive(Debug, Copy, Clone)]
enum InputType {
  Pack,
  Unpack,
}

impl Default for InputType {
  fn default() -> InputType {
    InputType::Pack
  }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(s2_grpc), supports(struct_named))]
pub struct InputReceiver {
  #[darling(skip)]
  input_type: InputType,
  ident: syn::Ident,
  generics: syn::Generics,
  data: ast::Data<(), FieldReceiver>,
  message_type: Paths,
}

impl InputReceiver {
  pub fn to_unpack(self) -> Self {
    Self {
      input_type: InputType::Unpack,
      ..self
    }
  }
}

impl ToTokens for InputReceiver {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let InputReceiver {
      input_type,
      ref ident,
      ref generics,
      ref data,
      ref message_type,
    } = *self;

    let (imp, ty, wher) = generics.split_for_impl();
    let fields = data
      .as_ref()
      .take_struct()
      .expect("Should never be enum")
      .fields;

    match input_type {
      InputType::Pack => {
        let pack_lines: Vec<_> = fields
          .iter()
          .map(|f| {
            let field_ident = &f.ident;
            let field_ty = &f.ty;
            let value_field_ident = if let Some(ident) = f.rename.as_ref() {
              ident
            } else {
              f.ident.as_ref().unwrap()
            };
            let value_expr = if let Some(ref path) = f.proto_enum_type {
              quote! { i32::from(<#field_ty as s2_grpc_utils::S2ProtoEnum<#path>>::into_proto_enum(value.#field_ident)) }
            } else {
              quote! { value.#field_ident }
            };
            if let Some(map_fn) = f.map_fn.as_ref() {
              quote! {
                #value_field_ident: #map_fn(#value_expr),
              }
            } else {
              quote! {
                #value_field_ident: #value_expr.pack()?,
              }
            }
          })
          .collect();
        for message_type in &message_type.paths {
          tokens.extend(quote! {
            impl #imp s2_grpc_utils::S2ProtoPack<#message_type> for #ident #ty #wher {
              fn pack(self) -> s2_grpc_utils::result::Result<#message_type> {
                let value = self;
                Ok(#message_type {
                  #(#pack_lines)*
                })
              }
            }

            impl #imp s2_grpc_utils::S2ProtoPack<Option<#message_type>> for #ident #ty #wher {
              fn pack(self) -> s2_grpc_utils::result::Result<Option<#message_type>> {
                let value = self;
                Ok(Some(#message_type {
                  #(#pack_lines)*
                }))
              }
            }
          })
        }
      }
      InputType::Unpack => {
        let unpack_lines: Vec<_> = fields
          .iter()
          .map(|f| {
            let field_ident = &f.ident;
            let field_ty = &f.ty;
            let value_field_ident = if let Some(ident) = f.rename.as_ref() {
              ident
            } else {
              f.ident.as_ref().unwrap()
            };
            let field_expr = if let Some(map_fn) = f.map_fn.as_ref() {
              if let Some(ref path) = f.proto_enum_type {
                quote! {
                  {
                    let value = #map_fn(value.#value_field_ident);
                    <#field_ty as s2_grpc_utils::S2ProtoEnum<#path>>::from_i32(value)
                      .ok_or_else(|| s2_grpc_utils::result::Error::EnumDiscriminantNotFound {
                        enum_name: <#field_ty as s2_grpc_utils::S2ProtoEnumMeta>::NAME,
                        discriminant: value,
                      })?
                  }
                }
              } else {
                quote! {
                  #map_fn(value.#value_field_ident)
                }
              }
            } else {
              if let Some(ref path) = f.proto_enum_type {
                quote! {
                  <#field_ty as s2_grpc_utils::S2ProtoEnum<#path>>::from_i32(value.#value_field_ident)
                    .ok_or_else(|| s2_grpc_utils::result::Error::EnumDiscriminantNotFound {
                      enum_name: <#field_ty as s2_grpc_utils::S2ProtoEnumMeta>::NAME,
                      discriminant: value.#value_field_ident,
                    })?
                }
              } else {
                quote! {
                  S2ProtoUnpack::unpack(value.#value_field_ident).map_err(|err| {
                    if let s2_grpc_utils::result::Error::ValueNotPresent = err {
                      s2_grpc_utils::result::Error::FieldValueNotPresent {
                        field_name: stringify!(#field_ident),
                      }
                    } else {
                      err
                    }
                  })?
                }
              }
            };
            quote! {
              #field_ident: #field_expr,
            }
          })
          .collect();

        for message_type in &message_type.paths {
          tokens.extend(quote! {
            impl #imp s2_grpc_utils::S2ProtoUnpack<#message_type> for #ident #ty #wher {
              fn unpack(value: #message_type) -> s2_grpc_utils::result::Result<#ident> {
                Ok(#ident {
                  #(#unpack_lines)*
                })
              }
            }

            impl #imp s2_grpc_utils::S2ProtoUnpack<Option<#message_type>> for #ident #ty #wher {
              fn unpack(value: Option<#message_type>) -> s2_grpc_utils::result::Result<#ident> {
                if let Some(value) = value {
                  Ok(#ident {
                    #(#unpack_lines)*
                  })
                } else {
                  Err(s2_grpc_utils::result::Error::ValueNotPresent)
                }
              }
            }
          })
        }
      }
    }
  }
}

#[derive(Debug, FromField)]
#[darling(attributes(s2_grpc))]
struct FieldReceiver {
  ident: Option<syn::Ident>,
  ty: syn::Type,
  #[darling(default)]
  rename: Option<syn::Ident>,
  #[darling(default)]
  map_fn: Option<syn::Path>,
  #[darling(default)]
  proto_enum_type: Option<syn::Path>,
}
