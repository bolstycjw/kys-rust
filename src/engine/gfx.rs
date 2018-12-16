use byteorder::{LittleEndian, ReadBytesExt};
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::fs;
use std::io::Cursor;
use std::io::Error;

struct Tile {
    x_offset: i16,
    y_offset: i16,
    index: u8,
}

pub fn load_index() -> Result<Vec<i16>, Error> {
    let buf = fs::read("./game/resource/smap/index.ka")?;
    let buf_size = buf.len() / 2;
    let mut rdr = Cursor::new(buf);
    let mut dst = vec![0; buf_size];
    rdr.read_i16_into::<LittleEndian>(&mut dst).unwrap();
    Ok(dst)
}

pub fn load_tiles<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    // for i in 0..700 {
    //     let empty_texture = texture_creator
    //         .load_texture("./game/resource/smap/0.png")
    //         .unwrap();
    //     let filename = format!("./game/resource/smap/{}.png", i);
    //     match texture_creator.load_texture(filename) {
    //         Ok(texture) => tiles.push(texture),
    //         _ => tiles.push(empty_texture),
    //     }
    // }
    tiles
}
