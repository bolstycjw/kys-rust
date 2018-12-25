use piston_window::*;

use super::state::State;

pub struct Title {}

impl State for Title {
    fn render(&self, window: &mut PistonWindow) -> G2dTexture {
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
        rust_logo
    }
}
