use graphics::{rectangle::square, types::Color};

use crate::{traits::Draw, DANGER_COLOR};

pub struct Cell {
    x: usize,
    y: usize,
    color: Color,
}

impl Cell {
    pub fn new(x: usize, y: usize, color: Color) -> Self {
        Self { x, y, color }
    }
}

impl Draw for Cell {
    fn draw(&self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        use graphics::*;

        rectangle(
            self.color,
            square(self.x as f64, self.y as f64, 1.0),
            c.transform,
            gl,
        );
    }
}
