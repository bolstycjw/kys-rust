extern crate find_folder;
extern crate piston_window;

mod config;
mod engine;
mod game;

use piston_window::*;

pub fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    window.set_lazy(true);

    let game = game::Game::new();
    game.run(&mut window);
}
