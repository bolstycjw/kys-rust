use piston_window::*;
use std::rc::Rc;

use super::math;
use super::math::Point;
use crate::game::tile::Tile;

pub fn draw_tile(tile: Rc<Tile>, pos: Point, e: &Event, w: &mut PistonWindow) {
    let (width, height) = tile.texture.get_size();
    let src_rect = [0.0, 0.0, width as f64, height as f64];
    let image = Image::new().src_rect(src_rect);
    let screen_pos = pos.to_iso();
    w.draw_2d(e, |c, g| {
        image.draw(
            &tile.texture,
            &c.draw_state,
            c.view.trans(
                screen_pos.x as f64 - tile.x_off as f64,
                screen_pos.y as f64 - tile.y_off as f64,
            ),
            g,
        )
    });
}
