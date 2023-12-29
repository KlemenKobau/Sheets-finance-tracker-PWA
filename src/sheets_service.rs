use google_sheets4::oauth2::ServiceAccountAuthenticator;
use tracing::{error, info};

use crate::errors::SheetsError;

pub async fn create_hub() -> Result<(), SheetsError> {
    info!("Creating hub");

    let secret = google_sheets4::oauth2::read_service_account_key("permissions.json")
        .await
        .map_err(|x| {
            error!("{x}");
            SheetsError::CredentialError
        })?;

    let auth = ServiceAccountAuthenticator::builder(secret)
        .build()
        .await
        .unwrap();

    Ok(())
}
