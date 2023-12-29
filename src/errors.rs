use thiserror::Error;

#[derive(Error, Debug)]
pub enum SheetsError {
    #[error("Credential error")]
    CredentialError,
}
