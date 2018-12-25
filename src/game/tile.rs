use byteorder::{LittleEndian, ReadBytesExt};
use std::fs;
use std::io::Cursor;
use std::io::Error;
use std::path::Path;

pub struct Tile {
    pub x_off: i16,
    pub y_off: i16,
    pub texture_paths: Vec<String>,
    pub frame_count: u8,
}

pub struct Tileset(pub Vec<Tile>);

pub fn load_tileset(tileset_path: &str) -> Tileset {
    let mut tiles = Vec::new();
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
        tiles.push(Tile {
            x_off: tile_index[i * 2],
            y_off: tile_index[i * 2 + 1],
            texture_paths,
            frame_count: frame_count,
        });
    }
    Tileset(tiles)
}

fn load_index(tileset_path: &str) -> Result<Vec<i16>, Error> {
    let buf = fs::read(format!("{}/index.ka", tileset_path))?;
    let buf_size = buf.len() / 2;
    let mut rdr = Cursor::new(buf);
    let mut dst = vec![0; buf_size];
    rdr.read_i16_into::<LittleEndian>(&mut dst).unwrap();
    Ok(dst)
}
