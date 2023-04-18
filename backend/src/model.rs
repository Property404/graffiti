use crate::api::{Color, Point, StateResponse, Update, Updates};
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
    pub tx: Sender<Updates>,
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
    pub async fn update_state(&self, Updates(updates): Updates) -> Result {
        if updates.len() > SIZE * SIZE {
            return Err(Error::RequestTooBig);
        }

        let mut state = self.state.lock().expect("poisoned");
        assert!(state.len() < (SIZE * SIZE));

        for Update { point, color } in updates {
            if point.x >= SIZE as u16 || point.y >= SIZE as u16 {
                continue;
            }
            state.insert(point, color);
        }

        Ok(())
    }

    pub async fn get_state(&self) -> Result<StateResponse> {
        let state = self.state.lock().expect("POISONED");
        Ok(StateResponse(state.clone()))
    }
}
