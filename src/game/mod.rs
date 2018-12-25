pub mod scene;
pub mod state;
pub mod tile;
pub mod title;

use std::boxed::Box;
use std::fs;
use std::io::Cursor;

use self::scene::Scene;
use self::state::State;
use self::title::Title;

pub struct Game {
    scenes: Vec<Scene>,
    state: Box<dyn State>,
}

impl Game {
    pub fn new() -> Self {
        {
            Self {
                scenes: Vec::new(),
                state: Box::new(Title {}),
            }
        }
    }

    pub fn change_state(&mut self, next_state: Box<dyn State>) {
        self.state = next_state;
    }

    pub fn run(self) {}

    pub fn load_scenes(&mut self) {
        let buf = fs::read("./bin/save/ALLSIN.GRP").unwrap();
        let mut rdr = Cursor::new(buf);

        for _i in 0..100 {
            let scene = Scene::read(&mut rdr);
            self.scenes.push(scene);
        }
    }
}
