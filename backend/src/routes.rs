use crate::api::{Color, Point, Update, Updates};
use crate::model::ModelController;
use crate::Result;
use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
    routing::{delete, get, post},
    Json, Router,
};
use futures_util::stream::{self, Stream};
use serde_json::json;
use std::{convert::Infallible, net::SocketAddr, time::Duration};
use tokio_stream::StreamExt;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/feed", get(sse_handler))
        .route("/update", post(update_state))
        .route("/state", get(get_state))
        .with_state(mc)
}

async fn update_state(State(mc): State<ModelController>, Json(updates): Json<Updates>) -> Result {
    mc.update_state(updates).await
}

async fn get_state(State(mc): State<ModelController>) -> Result<Json<Updates>> {
    let state = mc.get_state().await?;
    Ok(Json(state))
}

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}
