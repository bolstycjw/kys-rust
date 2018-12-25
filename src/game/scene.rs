use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

use super::state::State;

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
    ground: Layer,
    building: Layer,
    object: Layer,
    event: Layer,
    building_depth: Layer,
    object_depth: Layer,
}

impl Scene {
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
