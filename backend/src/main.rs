#![allow(unused, unused_mut)]
mod errors;
mod model;
mod web;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response, sse::{Sse, KeepAlive, Event}},
    routing::{get, get_service},
    Router,
};
use errors::{Error, Result};
use model::ModelController;
use serde::Deserialize;
use std::{time::Duration, net::SocketAddr, convert::Infallible};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tokio_stream::StreamExt;
use futures_util::stream::{self, Stream};

const STYLE: &str = "<style>html{background-color:black;color:white}</style>";

#[tokio::main]
async fn main() -> Result {
    let mc = ModelController::new().await?;

    let routes_merged = Router::new()
        .route("/sse", get(sse_handler))
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
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

async fn main_response_mapper(res: Response) -> Response {
    println!("<main_response_mapper>");
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello2/:name", get(hello2_handler))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// 'hello?name=Dagan'
async fn hello_handler(Query(HelloParams { name }): Query<HelloParams>) -> impl IntoResponse {
    let name = name.as_deref().unwrap_or("World!");
    Html(format!("{STYLE}Hello {name}!"))
}

// 'hello2/Dagan'
async fn hello2_handler(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("{STYLE}Hello {name}!"))
}
