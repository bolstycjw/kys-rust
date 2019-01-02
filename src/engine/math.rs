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

pub fn compute_bounds(pos: &Point) -> (usize, usize, usize, usize) {
    let start_x = cmp::max(pos.x - COL_TILE_COUNT as i32, 0);
    let start_y = cmp::max(pos.y - COL_TILE_COUNT as i32, 0);
    let end_x = cmp::min(pos.x + COL_TILE_COUNT as i32, 64);
    let end_y = cmp::min(pos.y + COL_TILE_COUNT as i32, 64);
    // (0, 64, 0, 64)
    (
        start_x as usize,
        end_x as usize,
        start_y as usize,
        end_y as usize,
    )
}
