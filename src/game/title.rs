use piston_window::*;

use super::scene::Scene;
use super::state::State;
use crate::config::*;

#[derive(Clone)]
pub struct Title {
    title: Option<G2dTexture>,
    next_state: Option<Box<dyn State>>,
}

impl Title {
    pub fn new() -> Self {
        Self {
            title: None,
            next_state: None,
        }
    }
}

impl State for Title {
    fn box_clone(&self) -> Box<dyn State> {
        Box::new((*self).clone())
    }

    fn handle_events(&mut self, event: &Event) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            println!("Pressed keybaord key '{:?}", key);
            self.next_state = Some(Box::new(Scene::load(0)));
        };
    }

    fn next_state(&self) -> Option<Box<dyn State>> {
        self.next_state.clone()
    }

    fn on_load(&mut self, w: &mut PistonWindow) {
        let resource = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("resource")
            .unwrap();
        let rust_logo = resource.join("open.png");
        let rust_logo: G2dTexture = Texture::from_path(
            &mut w.factory,
            &rust_logo,
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();
        self.title = Some(rust_logo);
    }

    fn render(&mut self, e: &Event, w: &mut PistonWindow) {
        if let Some(rust_logo) = &self.title {
            let (width, height) = rust_logo.get_size();
            let sx = SCREEN_WIDTH as f64 / width as f64;
            let sy = SCREEN_HEIGHT as f64 / height as f64;
            w.draw_2d(e, |c, g| {
                clear([1.0; 4], g);
                image(rust_logo, c.transform.scale(sx, sy), g);
            });
        }
    }
}
