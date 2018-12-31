use byteorder::{LittleEndian, ReadBytesExt};
use piston_window::*;
use std::fs::File;
use std::io::{Seek, SeekFrom};

use super::state::State;
use super::tile::TileManager;
use crate::config::*;
use crate::engine::math;

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

    fn on_load(&mut self, window: &mut PistonWindow) {
        let smap = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("smap")
            .unwrap();
        // self.tileset = Some(load_tileset(&smap, window));
    }

    fn render(&mut self, e: &Event, window: &mut PistonWindow) {
        let Scene {
            tile_manager,
            ground,
            ..
        } = self;
        for y in 0..64 {
            for x in 0..64 {
                let num = ground.0[y][x] as usize;
                let tile = tile_manager.load(num / 2, window).unwrap();
                let src_rect = [
                    tile.x_off as f64,
                    tile.y_off as f64,
                    TILE_WIDTH as f64,
                    TILE_HEIGHT as f64,
                ];
                let image = Image::new().src_rect(src_rect);
                let (iso_x, iso_y) = math::cart_to_iso(x as i32, y as i32);
                window.draw_2d(e, |c, g| {
                    image.draw(
                        &tile.texture,
                        &c.draw_state,
                        c.view.trans(iso_x as f64, iso_y as f64),
                        g,
                    )
                });
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
