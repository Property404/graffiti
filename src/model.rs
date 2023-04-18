use crate::api::{Color, Point, StateResponse, Update};
use crate::errors::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio_stream::wrappers::BroadcastStream;

const SIZE: usize = 1024;
const CHANNEL_WIDTH: usize = 32;

#[derive(Clone)]
pub struct ModelController {
    state: Arc<Mutex<HashMap<Point, Color>>>,
    pub tx: Sender<Update>,
}

impl Default for ModelController {
    fn default() -> Self {
        let (tx, _) = channel(CHANNEL_WIDTH);
        ModelController {
            state: Default::default(),
            tx,
        }
    }
}

impl ModelController {
    pub async fn update_state(&self, update: Update) -> Result {
        let mut state = self.state.lock().expect("poisoned");
        assert!(state.len() < (SIZE * SIZE));

        for (point, color) in update.to_map() {
            if point.x >= SIZE as u16 || point.y >= SIZE as u16 {
                continue;
            }
            state.insert(point, color);
        }

        Ok(())
    }

    pub async fn get_state(&self) -> Result<StateResponse> {
        let state = self.state.lock().expect("POISONED");
        let state = state.iter().map(|(a, b)| (*a, *b)).collect();
        Ok(StateResponse(state))
    }
}
