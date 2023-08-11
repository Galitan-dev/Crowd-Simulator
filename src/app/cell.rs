use std::cmp::Ordering;

use graphics::{rectangle::square, types::Color};

use crate::traits::Draw;

use super::grid::distance;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    x: usize,
    y: usize,
    color: Color,
}

impl Cell {
    pub fn new(x: usize, y: usize, color: Color) -> Self {
        Self { x, y, color }
    }

    pub fn pos(&self) -> [usize; 2] {
        [self.x, self.y]
    }

    pub fn go_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn targets(&self) -> Vec<[isize; 2]> {
        let mut targets = Vec::new();

        for dx in 0..3 {
            for dy in 0..3 {
                if dx == 1 && dy == 1 {
                    continue;
                }

                let x = self.x as isize - dx + 1;
                let y = self.y as isize - dy + 1;

                if distance(x, y) < distance(self.x as isize, self.y as isize) {
                    targets.push([x, y]);
                }
            }
        }

        targets.sort_by(|a, b| {
            if distance(b[0], b[1]) > distance(a[0], a[1]) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        targets
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
