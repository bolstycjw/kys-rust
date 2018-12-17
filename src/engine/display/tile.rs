use byteorder::{LittleEndian, ReadBytesExt};
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::fs;
use std::io::Cursor;
use std::io::Error;
use std::path::Path;

pub struct Tile<'a> {
    x_off: i16,
    y_off: i16,
    textures: Vec<Texture<'a>>,
    frame_count: u8,
}

fn load_tiles<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    let tile_index = load_index().unwrap();
    for i in 0..tile_index.len() {
        let path = format!("./game/resource/smap/{}.png", i);
        let mut frame_count = 0;
        let mut textures = Vec::new();
        if Path::new(&path).exists() {
            textures.push(texture_creator.load_texture(&path).unwrap());
            frame_count = 1;
        } else {
            let mut path = format!("./game/resource/smap/{}_{}.png", i, frame_count);
            while Path::new(&path).exists() {
                textures.push(texture_creator.load_texture(&path).unwrap());
                frame_count += 1;
                path = format!("./game/resource/smap/{}_{}.png", i, frame_count);
            }
        }
        tiles.push(Tile {
            x_off: tile_index[i * 2],
            y_off: tile_index[i * 2 + 1],
            textures: textures,
            frame_count: frame_count,
        });
    }
    tiles
}

fn load_index() -> Result<Vec<i16>, Error> {
    let buf = fs::read("./game/resource/smap/index.ka")?;
    let buf_size = buf.len() / 2;
    let mut rdr = Cursor::new(buf);
    let mut dst = vec![0; buf_size];
    rdr.read_i16_into::<LittleEndian>(&mut dst).unwrap();
    Ok(dst)
}
