use byteorder::{LittleEndian, ReadBytesExt};
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::*;
use std::fs;
use std::io::Cursor;

use super::state::State;
use super::tile::{load_tileset, Tileset};
use crate::config::*;

pub struct Layer(pub [[u16; 64]; 64]);

impl Layer {
    pub fn read(rdr: &mut Cursor<Vec<u8>>) -> Layer {
        let mut layer = [[0; 64]; 64];
        for i in 0..64 {
            let mut row = [0; 64];
            rdr.read_u16_into::<LittleEndian>(&mut row).unwrap();
            layer[i] = row;
        }
        Layer(layer)
    }
}

pub struct Scene {
    scenes: Vec<SceneData>,
    current: usize,
    tileset: Option<Tileset>,
    next_state: Option<Box<dyn State>>,
}

pub struct SceneData {
    ground: Layer,
    building: Layer,
    object: Layer,
    event: Layer,
    building_depth: Layer,
    object_depth: Layer,
}

impl State for Scene {
    fn handle_events(&mut self, event: &Event) {}

    fn next_state(&self) -> &Option<Box<dyn State>> {
        &self.next_state
    }

    fn on_load(&mut self, window: &mut PistonWindow) {
        let smap = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("smap")
            .unwrap();
        self.tileset = Some(load_tileset(&smap, window));
    }

    fn render(&self, c: &Context, g: &mut GfxGraphics<Resources, CommandBuffer>) {
        let SceneData { ground, .. } = &self.scenes[self.current];
        for y in 0..64 {
            for x in 0..64 {
                let num = ground.0[y][x] as usize;
                if let Some(tileset) = &self.tileset {
                    let tile = &tileset.0[num];
                    let src_rect = [
                        tile.x_off as f64,
                        tile.y_off as f64,
                        TILE_WIDTH as f64,
                        TILE_HEIGHT as f64,
                    ];
                };
            }
        }
    }
}

impl Scene {
    pub fn new() -> Self {
        let buf = fs::read("./bin/save/ALLSIN.GRP").unwrap();
        let mut rdr = Cursor::new(buf);
        let mut scenes = Vec::new();

        for _i in 0..100 {
            let scene = SceneData::read(&mut rdr);
            scenes.push(scene);
        }
        Self {
            scenes,
            current: 0,
            tileset: None,
            next_state: None,
        }
    }
}

impl SceneData {
    pub fn read(rdr: &mut Cursor<Vec<u8>>) -> Self {
        let ground = Layer::read(rdr);
        let building = Layer::read(rdr);
        let object = Layer::read(rdr);
        let event = Layer::read(rdr);
        let building_depth = Layer::read(rdr);
        let object_depth = Layer::read(rdr);
        Self {
            ground,
            building,
            object,
            event,
            building_depth,
            object_depth,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn internal() {
        let buf = fs::read("./bin/save/ALLSIN.GRP").unwrap();
        let mut rdr = Cursor::new(buf);
        let scene = Scene::read(&mut rdr);
        for y in 0..64 {
            for x in 0..64 {
                print!("{},", scene.object_depth.0[y][x]);
            }
            println!("")
        }
        assert!(false);
    }
}
