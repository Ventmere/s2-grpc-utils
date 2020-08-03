use darling::{Error, FromMeta};
use syn;

#[derive(Debug)]
pub struct Paths {
  pub paths: Vec<syn::Path>,
}

impl FromMeta for Paths {
  fn from_list(items: &[syn::NestedMeta]) -> Result<Self, Error> {
    let mut paths = vec![];
    for item in items {
      if let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = *item {
        paths.push(path.clone())
      } else {
        return Err(Error::custom("not a path").with_span(item));
      }
    }
    Ok(Self { paths })
  }

  fn from_string(value: &str) -> Result<Self, Error> {
    let path: syn::Path = syn::parse_str(value).map_err(Error::custom)?;
    Ok(Self { paths: vec![path] })
  }
}
