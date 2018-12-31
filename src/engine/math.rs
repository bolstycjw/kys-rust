use crate::config::*;

pub fn cart_to_iso(cart_x: i32, cart_y: i32) -> (i32, i32) {
    let x = HALF_TILE_WIDTH as i32 * (cart_x - cart_y) + SCREEN_WIDTH as i32 / 2 - 18;
    let y = HALF_TILE_HEIGHT as i32 * (cart_x + cart_y);
    (x, y)
}
