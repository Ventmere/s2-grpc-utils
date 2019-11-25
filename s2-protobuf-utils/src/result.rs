use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
  #[snafu(display("JSON error: {}", source))]
  Json { source: serde_json::Error },
  #[snafu(display("Could not convert json value from type_url: {}", type_url))]
  JsonTypeUrlUnknown { type_url: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
