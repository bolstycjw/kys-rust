use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::*;

use super::state::State;
use crate::config::*;

pub struct Title {
    title: Option<G2dTexture>,
}

impl Title {
    pub fn new() -> Self {
        Self { title: None }
    }
}

impl State for Title {
    fn on_load(&mut self, window: &mut PistonWindow) {
        let resource = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("resource")
            .unwrap();
        let rust_logo = resource.join("open.png");
        let rust_logo: G2dTexture = Texture::from_path(
            &mut window.factory,
            &rust_logo,
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();
        self.title = Some(rust_logo);
    }

    fn render(&self, c: &Context, g: &mut GfxGraphics<Resources, CommandBuffer>) {
        if let Some(rust_logo) = &self.title {
            let (width, height) = rust_logo.get_size();
            let sx = SCREEN_WIDTH as f64 / width as f64;
            let sy = SCREEN_HEIGHT as f64 / height as f64;
            image(rust_logo, c.transform.scale(sx, sy), g);
        }
    }
}
