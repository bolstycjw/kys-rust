use byteorder::{LittleEndian, ReadBytesExt};
use piston_window::*;
use std::fs;
use std::io::Cursor;
use std::io::Error;
use std::path::Path;

pub struct Tile {
    pub x_off: i16,
    pub y_off: i16,
    pub textures: Vec<G2dTexture>,
    pub frame_count: u8,
}

pub struct Tileset(pub Vec<Tile>);

pub fn load_tileset(tileset_path: &Path, window: &mut PistonWindow) -> Tileset {
    let mut tiles = Vec::new();
    let tile_index = load_index(tileset_path).unwrap();
    for i in 0..tile_index.len() / 2 {
        let pathname = format!("{}.png", i);
        let texture = tileset_path.join(pathname);
        let mut frame_count = 0;
        let mut textures = Vec::new();
        if texture.exists() {
            textures.push(
                Texture::from_path(
                    &mut window.factory,
                    texture,
                    Flip::None,
                    &TextureSettings::new(),
                )
                .unwrap(),
            );
            frame_count = 1;
        } else {
            let pathname = format!("{}_{}.png", i, frame_count);
            let mut texture = tileset_path.join(pathname);
            while texture.exists() {
                textures.push(
                    Texture::from_path(
                        &mut window.factory,
                        texture,
                        Flip::None,
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                );
                frame_count += 1;
                let pathname = format!("{}_{}.png", i, frame_count);
                texture = tileset_path.join(pathname);
            }
        }
        tiles.push(Tile {
            x_off: tile_index[i * 2],
            y_off: tile_index[i * 2 + 1],
            textures,
            frame_count: frame_count,
        });
    }
    Tileset(tiles)
}

fn load_index(tileset_path: &Path) -> Result<Vec<i16>, Error> {
    let buf = fs::read(tileset_path)?;
    let buf_size = buf.len() / 2;
    let mut rdr = Cursor::new(buf);
    let mut dst = vec![0; buf_size];
    rdr.read_i16_into::<LittleEndian>(&mut dst).unwrap();
    Ok(dst)
}
