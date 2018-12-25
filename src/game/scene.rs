use byteorder::{LittleEndian, ReadBytesExt};
use opengl_graphics::*;
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
    current: u16,
    tileset: Tileset,
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
    fn render(&self, c: &Context, g: &mut GlGraphics) {
        let SceneData { ground, .. } = self.scenes[self.current];
        for y in 0..64 {
            for x in 0..64 {
                let num = ground[y][x];
                let tile = tileset[num];
                let src_rect = [
                    tile.x_off as f64,
                    tile.y_off as f64,
                    TILE_WIDTH as f64,
                    TILE_HEIGHT as f64,
                ];
            }
        }
    }
}

impl Scene {
    pub fn new() -> Self {
        let tileset = load_tileset("./bin/resource/smap");
        let buf = fs::read("./bin/save/ALLSIN.GRP").unwrap();
        let mut rdr = Cursor::new(buf);
        let scenes = Vec::new();

        for _i in 0..100 {
            let scene = SceneData::read(&mut rdr);
            scenes.push(scene);
        }
        Self {
            scenes,
            current: 0,
            tileset,
        }
    }
}

impl SceneData {
    pub fn read(rdr: &mut Cursor<Vec<u8>>) -> Scene {
        let ground = Layer::read(rdr);
        let building = Layer::read(rdr);
        let object = Layer::read(rdr);
        let event = Layer::read(rdr);
        let building_depth = Layer::read(rdr);
        let object_depth = Layer::read(rdr);
        Scene {
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
