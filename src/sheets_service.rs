use google_sheets4::oauth2::ApplicationSecret;
use shuttle_secrets::SecretStore;
use tracing::info;

pub async fn create_hub(secret_store: SecretStore) {
    info!("Creating hub");

    // let secret = ApplicationSecret {};
    // info!("{:?}", secret);
}
