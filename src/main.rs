mod common;
mod oncalls;
mod type_definitions;
mod db;
mod system;
mod user;

use std::net::SocketAddr;
use std::time::Duration;
use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    timeout::TimeoutLayer,
    set_header::SetResponseHeaderLayer
};

use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post, put},
    Router,
};

use system::appstate::AppState;
use db::mongo::db_connection;
use oncalls::handlers::*;
use type_definitions::handlers::*;
use user::handlers::*;
use common::handlers::{handler_404, root};
use crate::system::config::get_config;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_state = AppState {
        db_client: db_connection().await.unwrap(),
        config: get_config()
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "rust_axum=debug,axum=debug,tower_http=debug,mongodb=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let server_header_value = HeaderValue::from_static("oncall-tracker");
    let _cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/admin/type-definition-group", get(list_type_definition_groups).post(insert_type_definition_group))
        .route("/admin/type-definition-group/:definition-group-id/definitions", post(create_type_definition_detail))
        .route("/admin/type-definition-group/:definition-group-id/definitions/:detail-id", put(update_type_definition_detail).delete(delete_type_definition_detail))
        .route("/admin/users", post(insert_user))
        .route("/oncalls", get(list_oncalls_years))
        .route("/oncalls/:year", get(list_oncalls_in_year))
        .route("/oncalls/:year/:month", get(on_call_details).post(save_day))
        .layer(TimeoutLayer::new(Duration::from_secs(3)))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::SERVER,
            server_header_value,
        ));

    let app = app
        .fallback(handler_404)
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8069));
    //tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}