use byteorder::{LittleEndian, ReadBytesExt};
use std::fs;
use std::io::Cursor;

pub struct Layer([[u16; 64]; 64]);

impl Layer {
    pub fn from(buf: &Vec<u8>) -> Layer {
        let mut rdr = Cursor::new(buf);
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
    pub fn handle_events() {}

    pub fn render() {}
}

pub fn load_scenes() {
    let buf = fs::read("./game/save/ALLSIN.GRP").unwrap();
    let layer = Layer::from(&buf);
    for y in 0..64 {
        for x in 0..64 {
            print!("{},", layer.0[y][x]);
        }
        println!("")
    }
}
