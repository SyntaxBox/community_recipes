use crate::controllers::auth_controller;
use axum::routing::{get, post};

use super::AppState;

pub fn create_auth_route(app_state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/login", post(auth_controller::login))
        .route("/register", post(auth_controller::register))
        .route("/logout", get(auth_controller::logout))
        .with_state(app_state)
}
