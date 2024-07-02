use crate::services::auth_service;
use axum::extract::Json;
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct LoginRequest {
    name: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
}

pub async fn login(Json(body): Json<LoginRequest>) -> Json<LoginResponse> {
    let message = auth_service::login(body.name, body.password);
    Json(LoginResponse { message })
}

pub async fn register() -> String {
    String::from("register")
}

pub async fn logout() -> String {
    String::from("logout")
}
