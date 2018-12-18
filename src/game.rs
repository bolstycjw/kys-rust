use crate::engine::Context;

pub fn run<'a, 'b: 'a>(ctx: &'a mut Context<'b>) {
    let disp_ctx = &mut ctx.disp_ctx;
    disp_ctx.load_tileset();
}
