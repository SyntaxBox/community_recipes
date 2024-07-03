pub mod controllers;
pub mod db;
pub mod routes;
pub mod services;

use routes::create_routes;
use sea_orm::Database;

pub async fn run(db_url: String) {
    let db = Database::connect(db_url).await.unwrap();
    // build our application with a single route
    let app = create_routes(db).await;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
