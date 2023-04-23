use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(default)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<Color> for u32 {
    fn from(Color { red, green, blue }: Color) -> Self {
        ((red as u32) << 16) + ((green as u32) << 8) + (blue as u32)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl From<Point> for u32 {
    fn from(Point { x, y }: Point) -> Self {
        0x80000000 + ((x as u32) << 16) + (y as u32)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Shape {
    /// This is a square. Can you guess what spot that goes in? That's right! It goes in the square
    /// hole
    Square,
    /// We also have this circle. Do you see a spot that would fit the circle? That's right! It's
    /// the square hole
    Circle,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Update {
    #[serde(flatten)]
    center: Point,
    radius: u16,
    color: Color,
    shape: Shape,
}

impl Update {
    pub fn into_map(self) -> HashMap<Point, Color> {
        let Self {
            center,
            radius,
            color,
            shape,
        } = self;
        let mut map = HashMap::new();
        match shape {
            Shape::Square => {
                for x in center.x.saturating_sub(radius)..center.x + radius {
                    for y in center.y.saturating_sub(radius)..center.y + radius {
                        map.insert(Point { x, y }, color);
                    }
                }
                map
            }
            Shape::Circle => {
                let min_x = center.x.saturating_sub(radius) as i16;
                let max_x = (center.x + radius) as i16;
                let min_y = (center.y.saturating_sub(radius)) as i16;
                let max_y = (center.y + radius) as i16;

                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        let relative_x = x - (center.x as i16);
                        let relative_y = y - (center.y as i16);
                        if relative_x * relative_x + relative_y * relative_y
                            <= (radius as i16 * radius as i16)
                        {
                            map.insert(
                                Point {
                                    x: x as u16,
                                    y: y as u16,
                                },
                                color,
                            );
                        }
                    }
                }

                map
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn conversions() {
        assert_eq!(u32::from(Point { x: 0, y: 0 }), 0x80000000);
        assert_eq!(
            u32::from(Point {
                x: 0x0234,
                y: 0x5678
            }),
            0x82345678
        );
        assert_eq!(
            u32::from(Point {
                x: 0x1234,
                y: 0x5678
            }),
            0x92345678
        );
    }
}
