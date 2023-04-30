use crate::api::{Color, Point, Update};
use crate::errors::Result;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast::{channel, Sender};

const SIZE: usize = 1024;
const CHANNEL_WIDTH: usize = 128;

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
    pub async fn update_state(&self, update: Update) -> Result<Option<Update>> {
        let mut state = self.state.lock().expect("poisoned");
        if state.len() > SIZE * SIZE {
            panic!("Bad length: {}", state.len());
        }

        let mut changed = false;

        let Update {
            center,
            radius,
            color,
        } = update;

        let mut effective_r = 0;

        for x in center.x.saturating_sub(radius)..center.x + radius {
            for y in center.y.saturating_sub(radius)..center.y + radius {
                let point = Point { x, y };
                if point.x >= SIZE as u16 || point.y >= SIZE as u16 {
                    continue;
                }

                if state.get(&point) != Some(&color) {
                    effective_r = std::cmp::max(effective_r, point.x.abs_diff(center.x));
                    effective_r = std::cmp::max(effective_r, point.y.abs_diff(center.y));
                    changed = true;
                    state.insert(point, color);
                }
            }
        }

        if changed {
            Ok(Some(Update {
                center,
                radius: effective_r,
                color,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_state(&self) -> Result<Vec<u8>> {
        let state = self.state.lock().expect("POISONED");
        let mut response = Vec::with_capacity(state.len());
        let mut last_color = None;
        for (point, color) in state.iter() {
            if last_color != Some(color) {
                last_color = Some(color);
                response.extend(u32::from(*color).to_le_bytes())
            }
            response.extend(u32::from(*point).to_le_bytes());
        }
        Ok(response)
    }
}
