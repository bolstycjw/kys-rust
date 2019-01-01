use byteorder::{LittleEndian, ReadBytesExt};
use piston_window::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;
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
        Self {
            path,
            cache: HashMap::new(),
        }
    }

    pub fn load(&mut self, tile_id: usize, w: &mut PistonWindow) -> Result<Rc<Tile>, String> {
        let Self { path, cache } = self;
        cache.get(&tile_id).cloned().map_or_else(
            || {
                let texture = path.join(format!("{}.png", tile_id));
                let mut index_file = File::open(path.join("index.ka")).unwrap();
                index_file
                    .seek(SeekFrom::Start(tile_id as u64 * 4))
                    .unwrap();
                let texture = Texture::from_path(
                    &mut w.factory,
                    &texture,
                    Flip::None,
                    &TextureSettings::new().mag(Filter::Nearest),
                )
                .unwrap();
                let tile = Tile {
                    x_off: index_file.read_i16::<LittleEndian>().unwrap(),
                    y_off: index_file.read_i16::<LittleEndian>().unwrap(),
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
