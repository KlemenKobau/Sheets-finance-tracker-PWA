use google_sheets4::{
    hyper::{self, client::HttpConnector},
    hyper_rustls::{self, HttpsConnector},
    oauth2::ServiceAccountAuthenticator,
    Sheets,
};
use tracing::error;

use crate::errors::SheetsError;

pub type SheetsConnector = Sheets<HttpsConnector<HttpConnector>>;

pub async fn create_hub() -> Result<SheetsConnector, SheetsError> {
    let secret = google_sheets4::oauth2::read_service_account_key("permissions.json")
        .await
        .map_err(|x| {
            error!("{x}");
            SheetsError::ServerCredentialError
        })?;

    let auth = ServiceAccountAuthenticator::builder(secret)
        .build()
        .await
        .map_err(|x| {
            error!("{x}");
            SheetsError::ServerCredentialError
        })?;

    let hub = Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http1()
                .build(),
        ),
        auth,
    );

    Ok(hub)
}
