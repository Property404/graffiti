mod api;
mod errors;
mod model;
mod routes;

use axum::{routing::get_service, Router};
use errors::{Error, Result};
use model::ModelController;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result {
    let mc = ModelController::default();

    let routes_merged = Router::new()
        .nest_service("/api", routes::routes(mc))
        .merge(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_merged.into_make_service())
        .await?;

    Ok(())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./frontend/")))
}
