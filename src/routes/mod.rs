use axum::routing::get;
pub mod auth_route;
pub fn create_routes() -> axum::Router {
    axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth_route::create_auth_route())
}
