pub struct Layer([[u16; 64]; 64]);

pub struct Scene {
    layers: [Layer; 6],
}

impl Scene {
    pub fn new() -> Scene {
        Scene {}
    }
}
