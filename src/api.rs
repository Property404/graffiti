use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Update {
    #[serde(flatten)]
    pub center: Point,
    pub radius: u16,
    pub color: Color,
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
