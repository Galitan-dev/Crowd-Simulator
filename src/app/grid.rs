use graphics::types::Color;
use rand::{thread_rng, Rng};

use crate::{traits::Draw, FOREGROUND_COLOR, SUCESS_COLOR};

use super::cell::Cell;

pub const COLUMNS: usize = 30;
pub const ROWS: usize = 30;

pub struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(cell_count: usize) -> Self {
        let mut cells = Vec::new();
        let mut rng = thread_rng();

        for i in 0..cell_count {
            let mut color: Color = rng.gen();
            color[3] = 1.0;
            cells.push(Cell::new(i % COLUMNS, i / ROWS, color));
        }

        Self { cells }
    }
}

impl Draw for Grid {
    fn draw(&self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        use graphics::*;

        for cell in &self.cells {
            cell.draw(c, gl);
        }

        let trans = c.transform.scale(COLUMNS as f64, ROWS as f64);

        line(FOREGROUND_COLOR, 0.01, [-0.01, 0.0, 1.0, 0.0], trans, gl);
        line(FOREGROUND_COLOR, 0.01, [0.0, 1.0, 1.01, 1.0], trans, gl);
        line(FOREGROUND_COLOR, 0.01, [0.0, 0.0, 0.0, 1.01], trans, gl);
        line(FOREGROUND_COLOR, 0.01, [1.0, -0.01, 1.0, 1.0], trans, gl);

        let middle = ROWS as f64 / 2.0;
        line(
            SUCESS_COLOR,
            0.01 * COLUMNS as f64,
            [0.0, middle, 0.0, middle + 2.0],
            c.transform,
            gl,
        );
    }
}
