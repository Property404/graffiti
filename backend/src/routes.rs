use crate::api::{Color, Point, StateResponse, Update};
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
use tokio::sync::mpsc::channel;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/feed", get(sse_handler))
        .route("/update", post(update_state))
        .route("/state", get(get_state))
        .with_state(mc)
}

async fn update_state(State(mc): State<ModelController>, Json(update): Json<Update>) -> Result {
    mc.tx.send(update.clone());
    mc.update_state(update).await
}

async fn get_state(State(mc): State<ModelController>) -> Result<Json<StateResponse>> {
    let state = mc.get_state().await?;
    Ok(Json(state))
}

async fn sse_handler(State(mc): State<ModelController>) -> Sse<impl Stream<Item = Result<Event>>> {
    let stream = BroadcastStream::new(mc.tx.subscribe())
        .map(|updates| Event::default().json_data(updates.unwrap()).unwrap())
        .map(Ok);

    Sse::new(stream).keep_alive(KeepAlive::default())
}
