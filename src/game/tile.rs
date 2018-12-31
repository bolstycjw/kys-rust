use byteorder::{LittleEndian, ReadBytesExt};
use piston_window::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Clone)]
pub struct Tile {
    pub x_off: i16,
    pub y_off: i16,
    pub texture: G2dTexture,
}

#[derive(Clone)]
pub struct TileManager {
    path: PathBuf,
    cache: HashMap<usize, Rc<Tile>>,
}

impl TileManager {
    pub fn new(tile_type: &str) -> Self {
        let path = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder(tile_type)
            .unwrap();
        let index = File::open(path.join("index.ka")).unwrap();
        Self {
            path,
            cache: HashMap::new(),
        }
    }

    pub fn load(&mut self, tile_id: usize, window: &mut PistonWindow) -> Result<Rc<Tile>, String> {
        let Self { path, cache } = self;
        cache.get(&tile_id).cloned().map_or_else(
            || {
                let texture = path.join(format!("{}.png", tile_id));
                let texture = Texture::from_path(
                    &mut window.factory,
                    &texture,
                    Flip::None,
                    &TextureSettings::new(),
                )
                .unwrap();
                let tile = Tile {
                    x_off: 0,
                    y_off: 0,
                    texture,
                };
                let tile = Rc::new(tile);
                cache.insert(tile_id, tile.clone());
                Ok(tile)
            },
            Ok,
        )
    }
}

// pub fn load_tileset(tileset_path: &Path, window: &mut PistonWindow) -> Tileset {
//     let mut tiles = Vec::new();
//     let tile_index = load_index(tileset_path).unwrap();
//     for i in 0..tile_index.len() / 2 {
//         let pathname = format!("{}.png", i);
//         let texture = tileset_path.join(pathname);
//         let mut frame_count = 0;
//         let mut textures = Vec::new();
//         if texture.exists() {
//             textures.push(
//                 Texture::from_path(
//                     &mut window.factory,
//                     texture,
//                     Flip::None,
//                     &TextureSettings::new(),
//                 )
//                 .unwrap(),
//             );
//             frame_count = 1;
//         } else {
//             let pathname = format!("{}_{}.png", i, frame_count);
//             let mut texture = tileset_path.join(pathname);
//             while texture.exists() {
//                 textures.push(
//                     Texture::from_path(
//                         &mut window.factory,
//                         texture,
//                         Flip::None,
//                         &TextureSettings::new(),
//                     )
//                     .unwrap(),
//                 );
//                 frame_count += 1;
//                 let pathname = format!("{}_{}.png", i, frame_count);
//                 texture = tileset_path.join(pathname);
//             }
//         }
//         tiles.push(Tile {
//             x_off: tile_index[i * 2],
//             y_off: tile_index[i * 2 + 1],
//             textures,
//             frame_count: frame_count,
//         });
//     }
//     Tileset(tiles)
// }

// fn load_index(tileset_path: &Path) -> Result<Vec<i16>, Error> {
//     let buf = fs::read(tileset_path)?;
//     let buf_size = buf.len() / 2;
//     let mut rdr = Cursor::new(buf);
//     let mut dst = vec![0; buf_size];
//     rdr.read_i16_into::<LittleEndian>(&mut dst).unwrap();
//     Ok(dst)
// }
