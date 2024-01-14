use anyhow::Context;
use api::{create_transaction, transaction_form};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{Response, StatusCode},
    middleware::{from_fn_with_state, Next},
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose, Engine};
use sheets_service::{create_hub, SheetsConnector};
use shuttle_secrets::SecretStore;
use tower_http::services::ServeDir;

mod api;
mod errors;
mod forms;
mod models {
    pub mod decimal;
}
mod sheets_service;

const AUTHORIZATION_HEADER: &str = "Authorization";
const AUTHENTICATE_HEADER: &str = "WWW-Authenticate";

#[derive(Clone)]
struct AppState {
    sheet_hub: SheetsConnector,
    sheet_id: String,
    app_password: String,
    app_username: String,
}

async fn password_check_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response<Body> {
    let auth_header = request.headers().get(AUTHORIZATION_HEADER);

    // getting credentials from Basic <base64 creds>
    let creds_string = auth_header
        .and_then(|x| x.to_str().ok())
        .and_then(|x| x.split_once(' '))
        .map(|x| x.1)
        .and_then(|x| general_purpose::STANDARD.decode(x).ok())
        .and_then(|x| String::from_utf8(x).ok());

    // splitting creds in form username:password
    let (username, password) = creds_string
        .as_ref()
        .and_then(|x| x.split_once(':'))
        .unwrap_or(("", ""));

    let is_username_correct = username == state.app_username;
    let is_password_correct = password == state.app_password;

    if is_username_correct && is_password_correct {
        next.run(request).await
    } else {
        let body = Body::from("Unauthorized");
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(AUTHENTICATE_HEADER, "Basic realm=\"Access to the website\"")
            .body(body)
            .expect("Unauthorized response not constructed properly")
    }
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let sheet_hub = create_hub().await?;
    let sheet_id = secret_store
        .get("SHEET_ID")
        .context("Sheet ID not configured.")?;

    let app_username = secret_store
        .get("APP_USERNAME")
        .context("App username not configured.")?;

    let app_password = secret_store
        .get("APP_PASSWORD")
        .context("App password not configured.")?;

    let app_state = AppState {
        sheet_hub,
        sheet_id,
        app_password,
        app_username,
    };

    let router = Router::new()
        .route("/", get(transaction_form))
        .route("/transactions", post(create_transaction))
        .route_layer(from_fn_with_state(
            app_state.clone(),
            password_check_middleware,
        ))
        .with_state(app_state)
        .nest_service("/static", ServeDir::new("static"));

    Ok(router.into())
}
