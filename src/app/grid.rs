use graphics::rectangle::square;

use crate::{traits::Draw, BACKGROUND_COLOR, SUCESS_COLOR};

pub struct Grid;

impl Draw for Grid {
    fn draw(&self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        use graphics::*;

        const SQUARE_SIZE: f64 = 0.25;

        clear(BACKGROUND_COLOR, gl);

        rectangle(
            SUCESS_COLOR,
            square(0.0, 0.0, SQUARE_SIZE),
            c.transform.trans(SQUARE_SIZE / -2.0, SQUARE_SIZE / -2.0),
            gl,
        );
    }
}
