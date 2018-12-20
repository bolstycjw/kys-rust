use crate::engine::Context;

pub struct Game<'a> {
    context: Context<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let context = Context::init();
        Game { context }
    }
}
