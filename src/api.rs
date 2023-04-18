use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(default)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Update {
    /// Single point update.
    Point { point: Point, color: Color },
    /// Rectangle range update.
    Rect {
        start: Point,
        end: Point,
        color: Color,
    },
}

impl Update {
    pub fn to_map(&self) -> HashMap<Point, Color> {
        match self {
            Self::Point { point, color } => HashMap::from([(*point, *color)]),
            Self::Rect { start, end, color } => {
                let mut map = HashMap::new();
                for x in start.x..end.x {
                    for y in start.y..end.y {
                        map.insert(Point { x, y }, *color);
                    }
                }
                map
            }
        }
    }
}

/// The whole board.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StateResponse(pub Vec<(Point, Color)>);
