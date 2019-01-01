use byteorder::{LittleEndian, ReadBytesExt};
use piston_window::*;
use std::fs::File;
use std::io::{Seek, SeekFrom};

use super::state::State;
use super::tile::{Tile, TileManager};
use crate::config::*;
use crate::engine::draw;

const LAYER_SIZE_BYTES: u64 = 64 * 64 * 2;
const SCENE_SIZE_BYTES: u64 = LAYER_SIZE_BYTES * 6;

#[derive(Clone)]
pub struct Layer(pub [[u16; 64]; 64]);

impl Layer {
    pub fn read(rdr: &mut File) -> Layer {
        let mut layer = [[0; 64]; 64];
        for i in 0..64 {
            let mut row = [0; 64];
            rdr.read_u16_into::<LittleEndian>(&mut row).unwrap();
            layer[i] = row;
        }
        Layer(layer)
    }
}

#[derive(Clone)]
pub struct Scene {
    ground: Layer,
    building: Layer,
    object: Layer,
    event: Layer,
    building_depth: Layer,
    object_depth: Layer,
    next_state: Option<Box<dyn State>>,
    tile_manager: TileManager,
}

impl State for Scene {
    fn box_clone(&self) -> Box<dyn State> {
        Box::new((*self).clone())
    }

    fn handle_events(&mut self, event: &Event) {}

    fn next_state(&self) -> Option<Box<dyn State>> {
        self.next_state.clone()
    }

    fn on_load(&mut self, w: &mut PistonWindow) {
        let smap = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("smap")
            .unwrap();
        // self.tileset = Some(load_tileset(&smap, w));
    }

    fn render(&mut self, e: &Event, w: &mut PistonWindow) {
        let Scene {
            tile_manager,
            ground,
            building,
            ..
        } = self;
        for y in 0..64 {
            for x in 0..64 {
                let tile_id = ground.0[y][x] as usize;
                let tile = tile_manager.load(tile_id / 2, w).unwrap();
                draw::draw_tile(tile, x as isize, y as isize, e, w);
                let tile_id = building.0[y][x] as usize;
                if tile_id > 0 {
                    let tile = tile_manager.load(tile_id / 2, w).unwrap();
                    draw::draw_tile(tile, x as isize, y as isize, e, w);
                }
            }
        }
    }
}

impl Scene {
    pub fn load(scene_id: usize) -> Self {
        let mut file = File::open("./bin/save/ALLSIN.GRP").unwrap();
        file.seek(SeekFrom::Start(scene_id as u64 * SCENE_SIZE_BYTES))
            .unwrap();
        let ground = Layer::read(&mut file);
        let building = Layer::read(&mut file);
        let object = Layer::read(&mut file);
        let event = Layer::read(&mut file);
        let building_depth = Layer::read(&mut file);
        let object_depth = Layer::read(&mut file);
        Self {
            ground,
            building,
            object,
            event,
            building_depth,
            object_depth,
            next_state: None,
            tile_manager: TileManager::new("smap"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn internal() {
        let mut file = File::open("./bin/save/ALLSIN.GRP").unwrap();
        file.seek(SeekFrom::Start(64 * 64 * 6 * 2));
        let scene = Scene::read(&mut file);
        for y in 0..64 {
            for x in 0..64 {
                print!("{},", scene.ground.0[y][x]);
            }
            println!("")
        }
        assert!(false);
    }
}
