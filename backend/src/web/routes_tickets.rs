use crate::api::{Color, Point, Update, Updates};
use crate::model::ModelController;
use crate::Result;
use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde_json::json;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
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
