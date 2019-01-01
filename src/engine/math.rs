use crate::config::*;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_iso(self) -> Self {
        let x = HALF_TILE_WIDTH as i32 * (self.x - self.y) + SCREEN_WIDTH as i32 / 2 - 18;
        let y = HALF_TILE_HEIGHT as i32 * (self.x + self.y);
        Self { x, y }
    }
}
