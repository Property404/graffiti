#![allow(unused)]
mod errors;
mod model;
mod routes;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service, post},
    Router,
};
pub use drawgan_api as api;
use errors::{Error, Result};
use model::ModelController;
use serde::Deserialize;
use std::{convert::Infallible, net::SocketAddr, time::Duration};
use tower_http::services::ServeDir;

const STYLE: &str = "<style>html{background-color:black;color:white}</style>";

#[tokio::main]
async fn main() -> Result {
    let mc = ModelController::default();

    let routes_merged = Router::new()
        .merge(routes::routes(mc))
        .merge(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_merged.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
