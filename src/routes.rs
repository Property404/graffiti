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
use http::header::{self, HeaderName, HeaderValue};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/state", get(get_state))
        .route("/feed", get(sse_handler))
        .route("/update", post(update_state))
        .with_state(mc)
}

async fn update_state(State(mc): State<ModelController>, Json(update): Json<Update>) -> Result {
    // Skip update if it makes no impact
    if let Some(update) = mc.update_state(update).await? {
        mc.tx.send(update).map_err(|e| Error::Send(e.to_string()))?;
    }
    Ok(())
}

async fn get_state(State(mc): State<ModelController>) -> Result<impl IntoResponse> {
    let state = mc.get_state().await?;
    Ok(state)
}

async fn sse_handler(State(mc): State<ModelController>) -> impl IntoResponse {
    let stream = BroadcastStream::new(mc.tx.subscribe()).map(|updates| {
        updates
            .map_err(|e| Error::Receive(e.to_string()))
            .and_then(|updates| Event::default().json_data(updates).map_err(Error::from))
    });

    let mut response = Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/event-stream"),
    );
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    response.headers_mut().insert(
        HeaderName::from_static("x-accel-buffering"),
        HeaderValue::from_static("no"),
    );
    response
}
