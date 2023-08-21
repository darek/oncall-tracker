use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};

use crate::common::struts::Error;
use crate::system::appstate::AppState;
use crate::oncalls::structs::*;
use crate::oncalls::repository::*;


pub async fn list_oncalls_years(State(app_state): State<AppState>) -> impl IntoResponse {
    let years: Vec<i32> = get_oncalls_years_for_user(
        app_state.db_client,
        app_state.config.mongo.unwrap())
        .await;
    
    (StatusCode::OK, Json(years))
}

pub async fn list_oncalls_in_year(State(app_state): State<AppState>, year: Path<u16>) -> impl IntoResponse {
    let rows: Vec<u8> = list_oncalls_in_year_for_user(
        app_state.db_client,
        app_state.config.mongo.unwrap(),
        year.0
    ).await;

    (StatusCode::OK, Json(rows))
}

pub async fn on_call_details(State(app_state): State<AppState>, params: Path<(u16, u8)>) -> impl IntoResponse {
    
    let on_call = get_on_call_details(
        app_state.db_client,
        app_state.config.mongo.unwrap(),
        params.0.0,
        params.0.1
    ).await;

    match on_call {
        Some(details) => (StatusCode::OK, Json(details)).into_response(),
        None => (StatusCode::NOT_FOUND, Json(Error { message: "Details not found".to_string()} )).into_response()
    }

}

pub async fn save_day(State(app_state): State<AppState>, params: Path<(u16, u8)>, Json(new_day): Json<OnCallDay>) -> impl IntoResponse {
    validate_oncall_day(new_day.clone());
    save_oncall_day(
        app_state.db_client,
        app_state.config.mongo.unwrap(),
    params.0.0,
    params.0.1,
    new_day).await;

    (StatusCode::OK, Json("OK"))
}

fn validate_oncall_day(on_call: OnCallDay) -> bool {
    if on_call.oncall_duration.is_some() && on_call.oncall_duration.unwrap() > 16.0 {
        return false
    }

    return true

}
