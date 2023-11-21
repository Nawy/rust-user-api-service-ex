use axum::{
    routing::{get, post},
    Json, Router,
};
use sea_orm::{Database, DatabaseConnection};
use std::{error::Error, net::SocketAddr};

mod api;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:postgres@localhost:5432/demo").await?;

    let app = Router::new()
        .route(
            "/user",
            get(api::users::get_all).post(api::users::create_user),
        )
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
