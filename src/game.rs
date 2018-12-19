use crate::engine::Context;

pub struct Game<'a> {
    context: Context<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let context = Context::init();
        Game { context }
    }

    pub fn load_assets(&mut self) {
        let disp_ctx = &mut self.context.display_context;
        disp_ctx.load_tileset();
    }

    pub fn load_data(&mut self) {}
}
