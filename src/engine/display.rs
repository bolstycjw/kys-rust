pub mod tile;

use self::tile::Tile;
use byteorder::{LittleEndian, ReadBytesExt};
use sdl2::image::LoadTexture;
use std::fs;
use std::io::Cursor;
use std::io::Error;
use std::path::Path;

use crate::context::Context;

pub fn load_tileset(ctx: &mut Context) {
    let mut tiles: Vec<Tile> = Vec::new();
    let tile_index = load_index().unwrap();
    for i in 0..tile_index.len() / 2 {
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

fn load_index() -> Result<Vec<i16>, Error> {
    let buf = fs::read("./game/resource/smap/index.ka")?;
    let buf_size = buf.len() / 2;
    let mut rdr = Cursor::new(buf);
    let mut dst = vec![0; buf_size];
    rdr.read_i16_into::<LittleEndian>(&mut dst).unwrap();
    Ok(dst)
}
