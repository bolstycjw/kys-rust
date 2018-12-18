pub mod tile;

use self::tile::{Tile, Tileset};
use byteorder::{LittleEndian, ReadBytesExt};
use sdl2::image::LoadTexture;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use std::fs;
use std::io::Cursor;
use std::io::Error;
use std::path::Path;

pub struct DisplayContext<'a> {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    tilesets: Vec<Tileset<'a>>,
}

impl<'a, 'b: 'a> DisplayContext<'b> {
    pub fn new(sdl_ctx: Sdl) -> DisplayContext<'a> {
        let video_subsystem = sdl_ctx.video().unwrap();
        let window = video_subsystem
            .window("kys rust", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        DisplayContext {
            canvas,
            texture_creator,
            tilesets: Vec::new(),
        }
    }

    pub fn load_tileset(&'a mut self) {
        let mut tiles: Vec<Tile> = Vec::new();
        let tile_index = load_index().unwrap();
        for i in 0..tile_index.len() {
            let pathname = format!("./game/resource/smap/{}.png", i);
            let mut frame_count = 0;
            let mut texture_paths = Vec::new();
            if Path::new(&pathname).exists() {
                texture_paths.push(pathname);
                frame_count = 1;
            } else {
                let mut pathname = format!("./game/resource/smap/{}_{}.png", i, frame_count);
                while Path::new(&pathname).exists() {
                    texture_paths.push(pathname);
                    frame_count += 1;
                    pathname = format!("./game/resource/smap/{}_{}.png", i, frame_count);
                }
            }
            let texture_creator = &self.texture_creator;
            let mut textures = Vec::new();
            for pathname in texture_paths.iter() {
                let texture = texture_creator.load_texture(&pathname).unwrap();
                textures.push(texture);
            }

            tiles.push(Tile {
                x_off: tile_index[i * 2],
                y_off: tile_index[i * 2 + 1],
                textures,
                frame_count: frame_count,
            });
        }
    }
}

fn load_index() -> Result<Vec<i16>, Error> {
    let buf = fs::read("./game/resource/smap/index.ka")?;
    let buf_size = buf.len() / 2;
    let mut rdr = Cursor::new(buf);
    let mut dst = vec![0; buf_size];
    rdr.read_i16_into::<LittleEndian>(&mut dst).unwrap();
    Ok(dst)
}
