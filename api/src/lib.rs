use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(default)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct Update {
    point: Point,
    color: Color,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct Updates(Vec<Update>);
