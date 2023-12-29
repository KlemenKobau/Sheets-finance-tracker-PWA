use anyhow::Context;
use google_sheets4::{
    hyper::{self, client::HttpConnector},
    hyper_rustls::{self, HttpsConnector},
    oauth2::ServiceAccountAuthenticator,
    Sheets,
};

pub type SheetsConnector = Sheets<HttpsConnector<HttpConnector>>;

pub async fn create_hub() -> anyhow::Result<SheetsConnector> {
    let secret = google_sheets4::oauth2::read_service_account_key("permissions.json")
        .await
        .context("Error reading sheet secrets.")?;

    let auth = ServiceAccountAuthenticator::builder(secret)
        .build()
        .await
        .context("Error creating authenticator.")?;

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
