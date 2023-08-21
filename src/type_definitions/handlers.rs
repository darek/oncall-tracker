use axum::{extract::{State}, http::StatusCode, response::IntoResponse, Json};
use axum::extract::Path;
use uuid::Uuid;

use crate::type_definitions::structs::{TypeDefinitionDetailRequest, NewTypeDefinitionGroup, TypeDefinitionDetails, TypeDefinitionGroup};
use crate::type_definitions::repository::{get_type_definition_groups, save_type_definition_detail, save_type_definition_group, insert_type_definition_detail, remove_type_definition_detail};
use crate::system::appstate::AppState;

pub async fn list_type_definition_groups(State(app_state): State<AppState>) -> impl IntoResponse {

    let definitions = get_type_definition_groups(
        app_state.db_client,
        app_state.config.mongo.unwrap()).await;

    (StatusCode::OK, Json(definitions))
}

pub async fn insert_type_definition_group(State(app_state): State<AppState>, Json(type_def_group): Json<NewTypeDefinitionGroup>) -> impl IntoResponse {
    let definition_group = TypeDefinitionGroup {
        id: Uuid::new_v4().to_string(),
        name: type_def_group.name,
        definitions: None
    };

    save_type_definition_group(
        app_state.db_client,
        app_state.config.mongo.unwrap(),
        definition_group.clone())
        .await;

    (StatusCode::OK, Json(definition_group))
}

pub async fn create_type_definition_detail(State(app_state): State<AppState>, type_definition_id: Path<String>, Json(definition_detail): Json<TypeDefinitionDetailRequest>) -> impl IntoResponse {
    let type_definition_detail = TypeDefinitionDetails {
        id: Uuid::new_v4().to_string(),
        name: definition_detail.name,
        base: definition_detail.base,
        multiplier: definition_detail.multiplier
    };

    insert_type_definition_detail(
        app_state.db_client,
        app_state.config.mongo.unwrap(),
        type_definition_id.0,
        type_definition_detail)
        .await;
        (StatusCode::OK, Json("OK"))
}

pub async fn update_type_definition_detail(State(app_state): State<AppState>, params: Path<(String, String)>, Json(definition_detail): Json<TypeDefinitionDetailRequest>) -> impl IntoResponse {
    let type_detail = TypeDefinitionDetails{
        id: params.0.1.clone(),
        name: definition_detail.name,
        base: definition_detail.base,
        multiplier: definition_detail.multiplier
    };

    save_type_definition_detail(
        app_state.db_client,
        app_state.config.mongo.unwrap(),
        params.0.0,
        params.0.1,
        type_detail)
        .await;
    (StatusCode::OK, Json("OK"))
}

pub async fn delete_type_definition_detail(State(app_state): State<AppState>, params: Path<(String, String)>) -> impl IntoResponse {
    remove_type_definition_detail(
        app_state.db_client,
        app_state.config.mongo.unwrap(),
        params.0.0,
        params.0.1)
        .await;
    (StatusCode::OK, Json("OK"))
}