use api::{create_transaction, transaction_form};
use axum::{
    routing::{get, post},
    Router,
};
use sheets_service::create_hub;
use shuttle_secrets::SecretStore;

mod api;
mod sheets_service;

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(transaction_form))
        .route("/transactions", post(create_transaction));

    create_hub(secret_store).await;

    Ok(router.into())
}
