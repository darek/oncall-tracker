use axum::{extract::{State}, response::IntoResponse, Json};
use uuid::Uuid;

use crate::user::struts::*;
use crate::user::repository::*;
use crate::system::appstate::AppState;

use argon2::{password_hash::{
    rand_core::OsRng,
    SaltString
}, Argon2, PasswordHasher};
use crate::user::struts::UserRole::USER;


pub async fn insert_user(State(app_state): State<AppState>, Json(new_user): Json<NewUser>) -> impl IntoResponse {
    if new_user.password != new_user.re_password {
        Err("password not match")
    } else {
        let user_by_name = get_user_by_username(app_state.db_client.clone(),
                                   app_state.config.mongo.clone().unwrap(),
                                   new_user.username.clone()).await;
        match user_by_name
        {
            Some(_user) => {
                Err("user ${new_user.username} exists")
            }
            None => {
                let salt = SaltString::generate(&mut OsRng);
                let argon2 = Argon2::default();
                let hash = argon2.hash_password(&new_user.password.as_bytes(), &salt).unwrap().hash.unwrap().to_string();

                let new_user_struct = User {
                    id: Uuid::new_v4().to_string(),
                    username: new_user.username,
                    password: hash,
                    role: USER
                };
                insert_new_user(app_state.db_client,
                            app_state.config.mongo.unwrap(),
                            new_user_struct.clone())
                .await;

                Ok("XXX")
            }

        }
    }
}