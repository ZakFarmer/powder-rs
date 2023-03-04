#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod config;
mod simulation;
mod utils;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use simulation::particle::ParticleVariant;
use simulation::timestep::{TimeStep, MS_PER_UPDATE};
use simulation::world::World;
use utils::graphics::Sprite;
use utils::loader::Assets;
use winit::window::Window;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const HEIGHT: u32 = 200;
const WIDTH: u32 = 200;

const LIGHT_PINK: [u8; 4] = [0xf2, 0x93, 0xb1, 0xff];
const PINK: [u8; 4] = [0xed, 0x51, 0x81, 0xff];
const RED: [u8; 4] = [0xe8, 0x2c, 0x45, 0xff];
const BLUE: [u8; 4] = [0x34, 0x56, 0x9d, 0xff];
const YELLOW: [u8; 4] = [0xff, 0xf9, 0x75, 0xff];
const DARK_YELLOW: [u8; 4] = [0xff, 0xea, 0x70, 0xff];
const ORANGE: [u8; 4] = [0xf8, 0xdb, 0x81, 0xff];

const COLORS: [[u8; 4]; 7] = [LIGHT_PINK, PINK, RED, BLUE, YELLOW, DARK_YELLOW, ORANGE];

fn main() -> Result<(), Error> {
    let variants: Vec<ParticleVariant> = vec![
        ParticleVariant::C4,
        ParticleVariant::DEUT,
        ParticleVariant::PLUT,
        ParticleVariant::STNE,
        ParticleVariant::URAN,
        ParticleVariant::WOOD,
    ];

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new(); // Why is this mutable?

    let window: Window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);

        WindowBuilder::new()
            .with_title("Powder Toy in Rust")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    //pixels.resize_buffer(WIDTH / 2, HEIGHT / 2);

    let mut world = World::new();
    let mut paused = false;

    let mut brush_size: i8 = 1;

    let assets: Assets = utils::loader::load_assets();
    let particle_sprite_data = assets
        .sprites()
        .get(&utils::graphics::Frame::Particle)
        .unwrap();

    let particle_sprite: Sprite = Sprite {
        pixels: particle_sprite_data.2.to_vec(),
        width: 1,
        height: 1,
    };

    let mut selected_particle_index: i8 = 2;

    let mut timestep = TimeStep::new();
    let mut lag = 0.0;

    event_loop.run(move |event, _, control_flow| {
        // Update lag
        lag += timestep.delta();

        if let Event::RedrawRequested(_) = event {
            // Clear the pixel buffer
            let frame = pixels.get_frame();
            for pixel in frame.chunks_exact_mut(4) {
                pixel[0] = 0x29; // R
                pixel[1] = 0x24; // G
                pixel[2] = 0x2b; // B
                pixel[3] = 0xff; // A
            }

            // Render the particles
            world.draw(pixels.get_frame(), &particle_sprite);

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.mouse_held(1) {
                let (mouse_x, mouse_y) = input.mouse().expect("Couldn't get mouse position!");

                for x in 0..brush_size {
                    for y in 0..brush_size {
                        world.add_particle(
                            (mouse_x / 6.0 + x as f32),
                            (mouse_y / 6.0 + y as f32),
                            variants[selected_particle_index as usize],
                        );
                    }
                }
            }

            if input.key_pressed(VirtualKeyCode::Comma) {
                if selected_particle_index == 0 {
                    selected_particle_index = 5; // Wrap back around to end of list
                } else {
                    selected_particle_index -= 1;
                }

                let element_string = match variants[selected_particle_index as usize] {
                    ParticleVariant::C4 => "C4",
                    ParticleVariant::DEUT => "DEUT",
                    ParticleVariant::PLUT => "PLUT",
                    ParticleVariant::STNE => "STNE",
                    ParticleVariant::URAN => "URAN",
                    ParticleVariant::WOOD => "WOOD",
                };

                println!("Switched to element: {}", element_string);
            } else if input.key_pressed(VirtualKeyCode::Period) {
                if selected_particle_index == 5 {
                    selected_particle_index = 0; // Wrap back around to start of list
                } else {
                    selected_particle_index += 1;
                }
            }

            if input.key_pressed(VirtualKeyCode::O) {
                if brush_size != 1 {
                    brush_size -= 1;
                }

                println!("Brush size: {}", brush_size);
            } else if input.key_pressed(VirtualKeyCode::P) {
                if brush_size != 20 {
                    brush_size += 1;
                }
                println!("Brush size: {}", brush_size);
            }

            if input.key_pressed(VirtualKeyCode::R) {
                world.clear_particles();
            }

            if input.key_pressed(VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(winit::event::VirtualKeyCode::Space) {
                paused = !paused;
            }
        }

        while lag >= MS_PER_UPDATE {
            world.update(MS_PER_UPDATE * 0.01);
            lag -= MS_PER_UPDATE;
        }

        window.request_redraw();
    });
}
