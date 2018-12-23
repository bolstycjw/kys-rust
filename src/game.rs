pub mod scene;
pub mod state;
pub mod tile;

use byteorder::{LittleEndian, ReadBytesExt};
use sdl2::image::LoadTexture;
use std::fs;
use std::io::Cursor;
use std::io::Error;
use std::path::Path;

use self::scene::{Layer, Scene};
use self::state::{EventHandler, Renderable, State};
use self::tile::{Tile, Tileset};
use crate::context::Context;

pub struct Game<'a> {
    context: Context,
    tilesets: Vec<Tileset<'a>>,
    scenes: Vec<Scene>,
    // cur_state: State,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let context = Context::init();

        Game {
            context,
            tilesets: Vec::new(),
            scenes: Vec::new(),
        }
    }

    pub fn run(&self) {
        // self.cur_state.handle_events();
        // self.cur_state.render();
    }

    pub fn load_scenes() {
        let buf = fs::read("./bin/save/ALLSIN.GRP").unwrap();
        // for y in 0..64 {
        //     for x in 0..64 {
        //         print!("{},", layer.0[y][x]);
        //     }
        //     println!("")
        // }
    }

    pub fn load_tileset(&self, tileset_path: &str) {
        let ctx = &self.context;
        let mut tiles: Vec<Tile> = Vec::new();
        let tile_index = load_index(tileset_path).unwrap();
        for i in 0..tile_index.len() / 2 {
            let pathname = format!("{}/{}.png", tileset_path, i);
            let mut frame_count = 0;
            let mut texture_paths = Vec::new();
            if Path::new(&pathname).exists() {
                texture_paths.push(pathname);
                frame_count = 1;
            } else {
                let mut pathname = format!("{}/{}_{}.png", tileset_path, i, frame_count);
                while Path::new(&pathname).exists() {
                    texture_paths.push(pathname);
                    frame_count += 1;
                    pathname = format!("{}/{}_{}.png", tileset_path, i, frame_count);
                }
            }
            let texture_creator = &ctx.texture_creator;
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

fn load_index(tileset_path: &str) -> Result<Vec<i16>, Error> {
    let buf = fs::read(format!("{}/index.ka", tileset_path))?;
    let buf_size = buf.len() / 2;
    let mut rdr = Cursor::new(buf);
    let mut dst = vec![0; buf_size];
    rdr.read_i16_into::<LittleEndian>(&mut dst).unwrap();
    Ok(dst)
}
