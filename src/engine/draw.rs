use piston_window::*;
use std::rc::Rc;

use super::math;
use crate::game::tile::Tile;

pub fn draw_tile(tile: Rc<Tile>, x: isize, y: isize, e: &Event, w: &mut PistonWindow) {
    let (width, height) = tile.texture.get_size();
    let src_rect = [0.0, 0.0, width as f64, height as f64];
    let image = Image::new().src_rect(src_rect);
    let (iso_x, iso_y) = math::cart_to_iso(x as i32, y as i32);
    w.draw_2d(e, |c, g| {
        image.draw(
            &tile.texture,
            &c.draw_state,
            c.view.trans(
                iso_x as f64 - tile.x_off as f64,
                iso_y as f64 - tile.y_off as f64,
            ),
            g,
        )
    });
}
