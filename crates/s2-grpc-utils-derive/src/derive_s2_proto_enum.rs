use darling::{ast, FromDeriveInput, FromVariant};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(s2_grpc), supports(enum_unit))]
pub struct InputReceiver {
  ident: syn::Ident,
  generics: syn::Generics,
  data: ast::Data<VariantReceiver, ()>,
  proto_enum_type: syn::Path,
}

impl ToTokens for InputReceiver {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let InputReceiver {
      ref ident,
      ref generics,
      ref data,
      ref proto_enum_type,
    } = *self;

    let (imp, ty, wher) = generics.split_for_impl();
    let variants = data.as_ref().take_enum().expect("Should never be struct");

    let (s2p, p2s): (Vec<_>, Vec<_>) = variants
      .iter()
      .map(|v| {
        let v_ident = &v.ident;
        (
          quote! {
            Self::#v_ident => #proto_enum_type::#v_ident,
          },
          quote! {
            #proto_enum_type::#v_ident => Some(Self::#v_ident),
          },
        )
      })
      .unzip();

    let names: Vec<_> = variants
      .iter()
      .map(|v| {
        let v_ident = &v.ident;
        let v_name = v.ident.to_string();
        quote! {
          Self::#v_ident => #v_name,
        }
      })
      .collect();

    let name = ident.to_string();

    tokens.extend(quote! {
      impl #imp s2_grpc_utils::S2ProtoEnum for #ident #ty  #wher {
        type ProtoEnum = #proto_enum_type;
        const NAME: &'static str = #name;

        fn from_i32(v: i32) -> Option<Self> {
          #proto_enum_type::from_i32(v)
            .and_then(|p| {
              match p {
                #(#p2s)*
                _ => None,
              }
            })
        }

        fn pack(&self) -> #proto_enum_type {
          match *self {
            #(#s2p)*
          }
        }

        fn get_variant_name(&self) -> &'static str {
          match *self {
            #(#names)*
          }
        }
      }
    })
  }
}

#[derive(Debug, FromVariant)]
struct VariantReceiver {
  ident: syn::Ident,
}
