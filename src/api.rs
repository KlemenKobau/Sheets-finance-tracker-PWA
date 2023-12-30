use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;
use fraction::Decimal;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{
    errors::AppError, forms::ValidatedForm, models::decimal::SerializableDecimal,
    sheets_service::add_transaction_to_sheet,
};

#[derive(Template)]
#[template(path = "transaction-form.html")]
pub struct TransactionTemplate;

pub async fn transaction_form() -> impl IntoResponse {
    TransactionTemplate
}

#[derive(Serialize, Deserialize, Validate)]
pub struct TransactionRequest {
    #[validate(custom = "validate_amount")]
    amount: SerializableDecimal,
}

fn validate_amount(
    SerializableDecimal(amount): &SerializableDecimal,
) -> Result<(), ValidationError> {
    if !amount.ge(&Decimal::from(0)) {
        return Err(ValidationError::new(
            "Amount should be more than 0 and should not be negative.",
        ));
    }
    Ok(())
}

pub async fn create_transaction(
    State(state): State<crate::AppState>,
    ValidatedForm(form): ValidatedForm<TransactionRequest>,
) -> Result<(), AppError> {
    add_transaction_to_sheet(state, form).await?;

    Ok(())
}
