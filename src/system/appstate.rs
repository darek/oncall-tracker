use axum::extract::FromRef;
use mongodb::Client;

use crate::system::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db_client: Client,
    pub config: Config
}

// support converting an `AppState` in an `ApiState`
impl FromRef<AppState> for Config {
    fn from_ref(app_state: &AppState) -> Config {
        app_state.config.clone()
    }
}
