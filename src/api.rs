use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{errors::AppError, forms::ValidatedForm, sheets_service::add_transaction_to_sheet};

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

#[derive(Serialize, Deserialize, Validate)]
pub struct TransactionRequest {
    #[validate(length(min = 1, message = "Can not be empty"))]
    amount: String,
}

pub async fn create_transaction(
    State(state): State<crate::AppState>,
    ValidatedForm(form): ValidatedForm<TransactionRequest>,
) -> Result<(), AppError> {
    add_transaction_to_sheet(state, form).await?;

    Ok(())
}
