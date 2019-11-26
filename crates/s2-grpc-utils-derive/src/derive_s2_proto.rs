use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(s2_proto), supports(struct_named))]
pub struct InputReceiver {
  ident: syn::Ident,
  generics: syn::Generics,
  data: ast::Data<(), FieldReceiver>,
  message_type: syn::Path,
}

impl ToTokens for InputReceiver {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let InputReceiver {
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

    let (pack_lines, unpack_lines): (Vec<_>, Vec<_>) = fields
      .iter()
      .map(|f| {
        let field_ident = &f.ident;
        (
          quote! {
            #field_ident: self.#field_ident.pack()?,
          },
          quote! {
            #field_ident: S2Proto::unpack(value.#field_ident)?,
          },
        )
      })
      .unzip();

    tokens.extend(quote! {
      impl #imp s2_protobuf_utils::S2Proto<#message_type> for #ident #ty #wher {
        fn pack(self) -> s2_protobuf_utils::result::Result<#message_type> {
          Ok(#message_type {
            #(#pack_lines)*
          })
        }
        fn unpack(value: #message_type) -> s2_protobuf_utils::result::Result<#ident> {
          Ok(#ident {
            #(#unpack_lines)*
          })
        }
      }
    })
  }
}

#[derive(Debug, FromField)]
#[darling(attributes(s2_proto))]
struct FieldReceiver {
  ident: Option<syn::Ident>,
  ty: syn::Type,
}
