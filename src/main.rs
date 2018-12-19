extern crate sdl2;

mod config;
mod engine;
mod game;

pub fn main() {
    let mut game = game::Game::new();
    game.load_assets();
    // game.load_data();
    // game.run();
}
