use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DetaError {
    #[error("Invalid project key")]
    InvalidProjectKey,
    #[error("Missing project key in environment variables")]
    MissingProjectKey,
}
