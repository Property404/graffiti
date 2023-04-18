use crate::api::{Color, Point, Update, Updates};
use crate::errors::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

const SIZE: usize = 256;

#[derive(Clone, Default)]
pub struct ModelController {
    state: Arc<Mutex<HashMap<Point, Color>>>,
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

    pub async fn get_state(&self) -> Result<Updates> {
        let state = self.state.lock().expect("POISONED");
        let updates = state
            .iter()
            .map(|(p, c)| Update {
                point: p.clone(),
                color: c.clone(),
            })
            .collect();
        Ok(Updates(updates))
    }
}
