extern crate sdl2;

use byteorder::{LittleEndian, ReadBytesExt};
use sdl2::event::Event;
use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const CENTER_X: i32 = SCREEN_WIDTH as i32 / 2;
const SCREEN_HEIGHT: u32 = 800;
const CENTER_Y: i32 = SCREEN_HEIGHT as i32 / 2;
const TILE_WIDTH: u32 = 36;
const HALF_TILE_WIDTH: u32 = TILE_WIDTH / 2;
const TILE_HEIGHT: u32 = 18;
const HALF_TILE_HEIGHT: u32 = TILE_HEIGHT / 2;
const IMAGE_WIDTH: u32 = 64 * TILE_WIDTH + SCREEN_WIDTH;
const IMAGE_HEIGHT: u32 = 64 * TILE_HEIGHT + SCREEN_HEIGHT;
const SCALED_TILE_WIDTH: u32 = IMAGE_WIDTH / 64;
const SCALED_TILE_HEIGHT: u32 = IMAGE_HEIGHT / 64;
const SCALED_HALF_TILE_WIDTH: u32 = SCALED_TILE_WIDTH / 2;
const SCALED_HALF_TILE_HEIGHT: u32 = SCALED_TILE_HEIGHT / 2;

pub fn cart_to_iso(cart: Point) -> Point {
    let x = SCALED_HALF_TILE_WIDTH as i32 * (cart.x - cart.y) + IMAGE_WIDTH as i32 / 2 - 18;
    let y = SCALED_HALF_TILE_HEIGHT as i32 * (cart.x + cart.y);
    Point::new(x, y)
}

pub fn load_png_tiles<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Vec<Texture> {
    let mut tiles: Vec<Texture> = Vec::new();
    for i in 0..700 {
        let empty_texture = texture_creator
            .load_texture("./game/resource/smap/0.png")
            .unwrap();
        let filename = format!("./game/resource/smap/{}.png", i);
        match texture_creator.load_texture(filename) {
            Ok(texture) => tiles.push(texture),
            _ => tiles.push(empty_texture),
        }
    }
    tiles
}

pub fn main() -> io::Result<()> {
    let mut scene = [[0; 64]; 64];
    let mut buf = vec![0; 1 * 6 * 64 * 64 * 2];
    let mut file = File::open("./game/save/ALLSIN.GRP")?;
    file.read(&mut buf);
    let mut rdr = Cursor::new(buf);
    for y in 0..64 {
        for x in 0..64 {
            scene[y][x] = rdr.read_u16::<LittleEndian>().unwrap();
            print!("{},", scene[y][x]);
        }
        println!();
    }

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
    let textures = load_png_tiles(&texture_creator);
    let mut target_texture = texture_creator
        .create_texture_target(None, IMAGE_WIDTH, IMAGE_HEIGHT)
        .unwrap();
    for y in 0..64 {
        for x in 0..64 {
            let loc = cart_to_iso(Point::new(x, y));
            let dest = Rect::new(loc.x, loc.y, SCALED_TILE_WIDTH, SCALED_TILE_HEIGHT);
            let num = (scene[y as usize][x as usize] / 2) as usize;
            canvas
                .with_texture_canvas(&mut target_texture, |texture_canvas| {
                    texture_canvas
                        .copy(&textures[num], None, dest)
                        .expect("Render failed");
                })
                .unwrap();
        }
    }
    let dest = Rect::new(800, 300, SCREEN_WIDTH, SCREEN_HEIGHT);
    canvas
        .copy(&target_texture, dest, None)
        .expect("Render failed");
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
    Ok(())
}
