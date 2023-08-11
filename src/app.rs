use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

use crate::{
    app::grid::{COLUMNS, ROWS},
    traits::{Draw, Update},
    BACKGROUND_COLOR,
};

pub use self::grid::Grid;

mod cell;
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
        let center = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let grid_scale = (
            args.window_size[0] * 0.8 / COLUMNS as f64,
            args.window_size[1] * 0.8 / ROWS as f64,
        );
        let grid_center = (COLUMNS as f64 / 2.0, ROWS as f64 / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            clear(BACKGROUND_COLOR, gl);

            self.grid.draw(
                c.trans_pos(center)
                    .scale_pos(grid_scale)
                    .trans(-grid_center.0, -grid_center.1),
                gl,
            );
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        self.grid.update();
    }
}
