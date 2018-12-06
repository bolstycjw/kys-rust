extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const CENTER_X: i32 = SCREEN_WIDTH as i32 / 2;
const SCREEN_HEIGHT: u32 = 600;
const CENTER_Y: i32 = SCREEN_HEIGHT as i32 / 2;
const TILE_WIDTH: u32 = 36;
const HALF_TILE_WIDTH: u32 = TILE_WIDTH / 2;
const TILE_HEIGHT: u32 = 18;
const HALF_TILE_HEIGHT: u32 = TILE_HEIGHT / 2;

pub fn cart_to_iso(cart: Point) -> Point {
    let x = HALF_TILE_WIDTH as i32 * (cart.x - cart.y) + CENTER_X - 18;
    let y = HALF_TILE_HEIGHT as i32 * (cart.x + cart.y);
    Point::new(x, y)
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG).unwrap();

    let window = video_subsystem
        .window("kys rust", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("./game/resource/mmap/9.png")
        .unwrap();
    let loc = cart_to_iso(Point::new(0, 0));
    let dest = Rect::new(loc.x, loc.y, TILE_WIDTH, TILE_HEIGHT);
    canvas.copy(&texture, None, dest).expect("Render failed");
    let loc = cart_to_iso(Point::new(0, 1));
    let dest = Rect::new(loc.x, loc.y, TILE_WIDTH, TILE_HEIGHT);
    canvas.copy(&texture, None, dest).expect("Render failed");
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
