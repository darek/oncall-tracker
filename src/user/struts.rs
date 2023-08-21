use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub role: UserRole
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    ADMIN,
    USER
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub re_password: String
}