use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

use crate::traits::Draw;

pub use self::grid::Grid;

mod grid;

pub struct App {
    gl: GlGraphics,
    grid: Grid,
}

impl App {
    pub fn new(gl: GlGraphics, grid: Grid) -> Self {
        Self { gl, grid }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let (sx, sy) = (args.window_size[0], args.window_size[1]);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            self.grid.draw(c.trans(x, y).scale(sx, sy), gl);
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
}
