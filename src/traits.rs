use graphics::Context;
use opengl_graphics::GlGraphics;

pub trait Draw {
    fn draw(&self, c: Context, gl: &mut GlGraphics);
}
