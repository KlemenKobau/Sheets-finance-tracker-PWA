use anyhow::Context;
use api::{create_transaction, transaction_form};
use axum::{
    routing::{get, post},
    Router,
};
use sheets_service::{create_hub, SheetsConnector};
use shuttle_secrets::SecretStore;

mod api;
mod errors;
mod forms;
mod models {
    pub mod decimal;
}
mod sheets_service;

#[derive(Clone)]
struct AppState {
    sheet_hub: SheetsConnector,
    sheet_id: String,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let sheet_hub = create_hub().await?;
    let sheet_id = secret_store
        .get("SHEET_ID")
        .context("Sheet ID not configured.")?;

    let app_state = AppState {
        sheet_hub,
        sheet_id,
    };

    let router = Router::new()
        .route("/", get(transaction_form))
        .route("/transactions", post(create_transaction))
        .with_state(app_state);

    Ok(router.into())
}
