use std::cmp;

use crate::config::*;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_iso(self) -> Self {
        let x = HALF_TILE_WIDTH as i32 * (self.x - self.y) + CENTER_X - 18;
        let y = HALF_TILE_HEIGHT as i32 * (self.x + self.y);
        Self { x, y }
    }

    pub fn relative(self, pos: Point) -> Self {
        let x = self.x - pos.x + CENTER_X;
        let y = self.y - pos.y + CENTER_Y;
        Self { x, y }
    }
}

pub fn within_bounds(x: i32, y: i32) -> bool {
    x >= 0 && x < 64 && y >= 0 && y < 64
}
