use askama::Template;
use askama_axum::IntoResponse;
use axum::Form;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "status.html")]
pub struct StatusTemplate;

pub async fn status() -> impl IntoResponse {
    StatusTemplate
}

#[derive(Template)]
#[template(path = "transaction-form.html")]
pub struct TransactionTemplate;

pub async fn transaction_form() -> impl IntoResponse {
    TransactionTemplate
}

#[derive(Serialize, Deserialize)]
pub struct TransactionRequest {
    amount: String,
}

pub async fn create_transaction(Form(form): Form<TransactionRequest>) {}
