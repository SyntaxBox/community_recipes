use crate::controllers::auth_controller;
use axum::routing::{get, post};

pub fn create_auth_route() -> axum::Router {
    axum::Router::new()
        .route("/login", post(auth_controller::login))
        .route("/register", post(auth_controller::register))
        .route("/logout", get(auth_controller::logout))
}
