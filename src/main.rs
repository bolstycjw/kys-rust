extern crate sdl2;

use byteorder::{LittleEndian, ReadBytesExt};
use sdl2::event::Event;
use sdl2::image::INIT_PNG;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::time::Duration;

use self::config::*;

mod config;
mod engine;

pub fn main() -> io::Result<()> {
    let mut scene = [[0; 64]; 64];
    let mut buf = vec![0; 1 * 6 * 64 * 64 * 2];
    let mut file = File::open("./game/save/ALLSIN.GRP")?;
    file.read(&mut buf);
    let mut rdr = Cursor::new(buf);
    for y in 0..64 {
        for x in 0..64 {
            scene[y][x] = rdr.read_u16::<LittleEndian>().unwrap();
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
    // let textures = engine::gfx::load_tiles(&texture_creator);
    // let mut target_texture = texture_creator
    //     .create_texture_target(None, IMAGE_WIDTH, IMAGE_HEIGHT)
    //     .unwrap();
    // for y in 0..64 {
    //     for x in 0..64 {
    //         let loc = engine::math::cart_to_iso(Point::new(x, y));
    //         let dest = Rect::new(loc.x, loc.y, SCALED_TILE_WIDTH, SCALED_TILE_HEIGHT);
    //         let num = (scene[y as usize][x as usize] / 2) as usize;
    //         canvas
    //             .with_texture_canvas(&mut target_texture, |texture_canvas| {
    //                 texture_canvas
    //                     .copy(&textures[num], None, dest)
    //                     .expect("Render failed");
    //             })
    //             .unwrap();
    //     }
    // }
    // let dest = Rect::new(800, 300, SCREEN_WIDTH, SCREEN_HEIGHT);
    // canvas
    //     .copy(&target_texture, dest, None)
    //     .expect("Render failed");
    // canvas.present();
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
