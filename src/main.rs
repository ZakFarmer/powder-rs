use raylib::prelude::*;
use sim::{
    grid::Grid,
    particle::{Element, Particle},
};

mod sim;

const GRAVITY: f32 = 0.1;
// const SCALE_FACTOR: f32 = 1.0;
const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;

fn main() {
    let mut grid = Grid::new(WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize);

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Sand")
        .build();

    while !rl.window_should_close() {
        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse_x = rl.get_mouse_x();
            let mouse_y = rl.get_mouse_y();

            let particle = Particle::new(Element::Sand);

            grid.set(mouse_x as usize, mouse_y as usize, particle);
        }

        grid.update();

        let mut draw = rl.begin_drawing(&thread);

        grid.draw(&mut draw);

        draw.clear_background(Color::BLACK);
    }
}
