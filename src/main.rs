extern crate sdl2;

mod config;
mod context;
mod engine;
mod game;

pub fn main() {
    let mut game = game::Game::new();
    game.load_tileset("./bin/resource/smap/");
    // game.run();
}
