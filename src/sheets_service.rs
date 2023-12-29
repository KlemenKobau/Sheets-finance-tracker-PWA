use anyhow::Context;
use google_sheets4::{
    api::ValueRange,
    hyper::{self, client::HttpConnector},
    hyper_rustls::{self, HttpsConnector},
    oauth2::ServiceAccountAuthenticator,
    Sheets,
};
use serde_json::json;
use tracing::error;

use crate::{api::TransactionRequest, AppState};

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

pub async fn add_transaction_to_sheet(
    state: AppState,
    form: TransactionRequest,
) -> anyhow::Result<()> {
    let hub = state.sheet_hub;
    let sheet_id = state.sheet_id;

    let values = vec![vec![json!("a"); 3]];

    let req = ValueRange {
        values: Some(values),
        ..Default::default()
    };

    hub.spreadsheets()
        .values_append(req, &sheet_id, "Transactions!A:C")
        .include_values_in_response(true)
        .value_input_option("RAW")
        .response_value_render_option("FORMATTED_VALUE")
        .response_date_time_render_option("FORMATTED_STRING")
        .doit()
        .await
        .map_err(|x| {
            error!("{x}");
            x
        })
        .context("Error writing to spreadsheet.")?;

    Ok(())
}
