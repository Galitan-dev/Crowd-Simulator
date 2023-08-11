use graphics::types::Color;
use rand::{thread_rng, Rng};

use crate::{
    traits::{Draw, Update},
    FOREGROUND_COLOR, SUCESS_COLOR,
};

use super::cell::Cell;

pub const COLUMNS: usize = 100;
pub const ROWS: usize = 100;
pub const TARGET: [usize; 2] = [0, ROWS / 2];

pub fn distance(x: isize, y: isize) -> f64 {
    (((TARGET[0] as isize - x).pow(2) + (TARGET[1] as isize - y).pow(2)) as f64).sqrt()
}

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

impl Update for Grid {
    fn update(&mut self) {
        let mut occupied: Vec<[usize; 2]> = self.cells.iter().map(|cell| cell.pos()).collect();

        let mut cells = self
            .cells
            .iter()
            .filter(|cell| distance(cell.pos()[0] as isize, cell.pos()[1] as isize) > 0.0)
            .copied()
            .collect::<Vec<_>>();

        for (i, cell) in cells.iter_mut().enumerate() {
            for target in cell.targets() {
                if target[0] < 0 || target[1] < 0 {
                    continue;
                }

                let (target_x, target_y) = (target[0] as usize, target[1] as usize);

                if target_x < COLUMNS
                    && target_y < ROWS
                    && !occupied.contains(&[target_x, target_y])
                    && (target_x != 2 || distance(target_x as isize, target_y as isize) > 3.0)
                {
                    cell.go_to(target_x, target_y);
                    occupied[i] = [target_x, target_y];
                    break;
                }
            }
        }

        self.cells = cells;
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

        line(
            SUCESS_COLOR,
            0.01 * COLUMNS as f64,
            [0.0, TARGET[1] as f64, 0.0, TARGET[1] as f64 + 1.0],
            c.transform,
            gl,
        );
    }
}
