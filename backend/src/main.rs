#![allow(unused, unused_mut)]
mod errors;
mod model;
mod web;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{
        sse::{Event, KeepAlive, Sse},
        Html, IntoResponse, Response,
    },
    routing::{get, get_service, post},
    Router,
};
pub use drawgan_api as api;
use errors::{Error, Result};
use futures_util::stream::{self, Stream};
use model::ModelController;
use serde::Deserialize;
use std::{convert::Infallible, net::SocketAddr, time::Duration};
use tokio_stream::StreamExt;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

const STYLE: &str = "<style>html{background-color:black;color:white}</style>";

#[tokio::main]
async fn main() -> Result {
    let mc = ModelController::default();

    let routes_merged = Router::new()
        .route("/sse", get(sse_handler))
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc))
        .merge(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_merged.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        // update state
        .route("/update", post(update_handler))
}

async fn update_handler(Query(api::Updates(updates)): Query<api::Updates>) -> impl IntoResponse {
    Html(format!("{STYLE}Hello world!"))
}
