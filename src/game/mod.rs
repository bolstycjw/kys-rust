pub mod scene;
pub mod state;
pub mod tile;
pub mod title;

use piston_window::*;
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

    pub fn run(&self, window: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let rust_logo = assets.join("rust.png");
        let rust_logo: G2dTexture = Texture::from_path(
            &mut window.factory,
            &rust_logo,
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&rust_logo, c.transform, g);
            });
        }
    }

    pub fn load_scenes(&mut self) {
        let buf = fs::read("./bin/save/ALLSIN.GRP").unwrap();
        let mut rdr = Cursor::new(buf);

        for _i in 0..100 {
            let scene = Scene::read(&mut rdr);
            self.scenes.push(scene);
        }
    }
}
