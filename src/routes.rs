use crate::api::Update;
use crate::model::ModelController;
use crate::{Error, Result};
use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
    routing::{get, post},
    Json, Router,
};
use futures_util::stream::Stream;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/state", get(get_state))
        .route("/feed", get(sse_handler))
        .route("/update", post(update_state))
        .with_state(mc)
}

async fn update_state(State(mc): State<ModelController>, Json(update): Json<Update>) -> Result {
    mc.tx
        .send(update.clone())
        .map_err(|e| Error::Send(e.to_string()))?;
    mc.update_state(update).await
}

async fn get_state(State(mc): State<ModelController>) -> Result<impl IntoResponse> {
    let state = mc.get_state().await?;
    Ok(state)
}

async fn sse_handler(State(mc): State<ModelController>) -> Sse<impl Stream<Item = Result<Event>>> {
    let stream = BroadcastStream::new(mc.tx.subscribe()).map(|updates| {
        Event::default()
            .json_data(updates.unwrap())
            .map_err(Error::from)
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
