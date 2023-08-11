extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use app::{App, Grid};
use glutin_window::GlutinWindow as Window;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod app;
mod traits;

const WIDTH: u32 = 900;
const HEIGHT: u32 = WIDTH / 16 * 9;

const FF: f32 = 255.0;
pub const FOREGROUND_COLOR: Color = [238.0 / FF, 244.0 / FF, 212.0 / FF, 1.0];
pub const BACKGROUND_COLOR: Color = [46.0 / FF, 53.0 / FF, 50.0 / FF, 1.0];
pub const SUCESS_COLOR: Color = [0.0 / FF, 168.0 / FF, 150.0 / FF, 1.0];
pub const DANGER_COLOR: Color = [255.0 / FF, 168.0 / FF, 95.0 / FF, 1.0];

const CELL_COUNT: usize = 4000;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Crowd Simulator", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(opengl), Grid::new(CELL_COUNT));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
