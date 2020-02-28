use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
  #[snafu(display("JSON error: {}", source))]
  Json { source: serde_json::Error },
  #[snafu(display("Could not convert json value from type_url: {}", type_url))]
  JsonTypeUrlUnknown { type_url: String },
  #[snafu(display("Could not unpack a non-optional value from null"))]
  ValueNotPresent,
  #[snafu(display("Could not unpack field '{}' from null", field_name))]
  FieldValueNotPresent { field_name: &'static str },
  #[snafu(display("JSON value nested too deeply"))]
  JsonValueNestedTooDeeply,
  #[snafu(display("List element {}: {}", index, source))]
  ListElement { source: Box<Error>, index: usize },
  #[snafu(display("Map entry: {}", source))]
  MapEntry { source: Box<Error> },
  #[snafu(display("Parse decimal error: {}", source))]
  ParseBigDecimal {
    source: bigdecimal::ParseBigDecimalError,
  },
  #[snafu(display(
    "Enum discriminant is not found: enum type = {}, discriminant = {}",
    enum_name,
    discriminant
  ))]
  EnumDiscriminantNotFound {
    enum_name: &'static str,
    discriminant: i32,
  },
}

impl From<Error> for String {
  fn from(e: Error) -> String {
    format!("{}", e)
  }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
