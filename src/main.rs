extern crate sdl2;

mod config;
mod engine;
mod game;

pub fn main() {
    let mut ctx = engine::Context::new();
    game::run(&mut ctx);
}
