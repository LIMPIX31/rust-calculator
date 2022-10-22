use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenizeError {
  #[error("No Matches")]
  NoMatches,
  #[error("Unknown character {0}")]
  UnknownChar(String),
  #[error("Unexpected EOF")]
  UnexpectedEOF
}
