extern crate sdl2;

mod config;
mod context;
mod engine;
mod game;

use context::Context;

pub fn main() {
    let ctx = Context::init();
    engine::load_assets(ctx);
    // let mut game = game::Game::new(ctx);
    // game.run();
}
