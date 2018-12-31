extern crate find_folder;
extern crate gfx_device_gl;
extern crate piston_window;

mod config;
mod engine;
mod game;

use self::config::*;
use piston_window::*;

pub fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("piston: image", [SCREEN_WIDTH, SCREEN_HEIGHT])
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

    window.set_lazy(true);

    let mut game = game::Game::new();
    game.run(&mut window);
}
