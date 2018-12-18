use sdl2::render::Texture;

pub struct Tile<'a> {
    pub x_off: i16,
    pub y_off: i16,
    pub textures: Vec<Texture<'a>>,
    pub frame_count: u8,
}

pub struct Tileset<'a>(Vec<Tile<'a>>);
