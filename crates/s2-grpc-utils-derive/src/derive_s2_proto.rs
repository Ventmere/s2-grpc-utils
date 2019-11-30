use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

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
  message_type: syn::Path,
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
            quote! {
              #field_ident: self.#field_ident.pack()?,
            }
          })
          .collect();
        tokens.extend(quote! {
          impl #imp s2_grpc_utils::S2ProtoPack<#message_type> for #ident #ty #wher {
            fn pack(self) -> s2_grpc_utils::result::Result<#message_type> {
              Ok(#message_type {
                #(#pack_lines)*
              })
            }
          }
        })
      }
      InputType::Unpack => {
        let unpack_lines: Vec<_> = fields
          .iter()
          .map(|f| {
            let field_ident = &f.ident;
            quote! {
              #field_ident: S2ProtoUnpack::unpack(value.#field_ident).map_err(|err| {
                if let s2_grpc_utils::result::Error::ValueNotPresent = err {
                  s2_grpc_utils::result::Error::FieldValueNotPresent {
                    field_name: stringify!(#field_ident),
                  }
                } else {
                  err
                }
              })?,
            }
          })
          .collect();
        tokens.extend(quote! {
          impl #imp s2_grpc_utils::S2ProtoUnpack<#message_type> for #ident #ty #wher {
            fn unpack(value: #message_type) -> s2_grpc_utils::result::Result<#ident> {
              Ok(#ident {
                #(#unpack_lines)*
              })
            }
          }
        })
      }
    }
  }
}

#[derive(Debug, FromField)]
struct FieldReceiver {
  ident: Option<syn::Ident>,
  ty: syn::Type,
}
