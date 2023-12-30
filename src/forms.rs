use axum::{
    async_trait,
    extract::{rejection::FormRejection, FromRequest, Request},
    Form,
};
use serde::de::DeserializeOwned;
use tracing::warn;
use validator::Validate;

use crate::errors::AppError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await.map_err(|x| {
            warn!("Form rejection: {x}");
            x
        })?;
        value.validate().map_err(|x| {
            warn!("Form validation failure: {x}");
            x
        })?;
        Ok(ValidatedForm(value))
    }
}
