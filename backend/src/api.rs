use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(default)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Update {
    pub point: Point,
    pub color: Color,
}

/// List of updates to apply.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Updates(pub Vec<Update>);

/// The whole board.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StateResponse(pub HashMap<Point, Color>);
