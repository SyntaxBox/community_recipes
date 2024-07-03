use axum::Router;
use sea_orm::DatabaseConnection;
pub mod auth_route;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn create_routes(db: DatabaseConnection) -> Router {
    let app_state = AppState { db };
    Router::new().nest("/auth", auth_route::create_auth_route(app_state))
}
