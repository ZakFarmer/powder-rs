use std::time::{Instant, Duration};

use graphics::graphics::{GraphicsContext};
use simulation::{simulation::Simulation, particle::Particle};
use winit::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder, WindowId}, event::{Event, WindowEvent}};

mod graphics;
mod math;
mod simulation;

const NUM_PARTICLES: usize = 1000000;

async fn run() {
    // Initialise event loop and window
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let window_id = window.id();

    let mut graphics = GraphicsContext::new(window).await;

    let window_size = graphics.window().inner_size();

    let update_interval = Duration::from_millis(1000 / 60);

    // Initialise simulation
    let gravity = -9.81;
    let time_step = 0.1;
    let damping = 0.;
    let bounds = [0.0, 0.0, window_size.width as f32, window_size.height as f32];
    let mut simulation = Simulation::new(gravity, time_step, damping, bounds);

    for _ in 0..NUM_PARTICLES {
        let x = rand::random::<f32>() * window_size.width as f32;
        let y = rand::random::<f32>() * window_size.height as f32;
        let vx = 0.;
        let vy = 0.;
        let radius = rand::random::<f32>() * 10.0 + 5.0;
        let color = [rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0];

        let particle = Particle::new(
            math::vector::Vector2d::new(x, y),
            math::vector::Vector2d::new(vx, vy),
            math::vector::Vector2d::new(0.0, 0.0),
            1.0,
            radius,
            color,
        );

        simulation.add_particle(particle);
    }

    let mut next_frame_time = Instant::now() + update_interval;
    
    let mut left_mouse_button_pressed = false;
    let mut mouse_position = (0.0, 0.0);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { 
                event: WindowEvent::CloseRequested, 
                window_id: id,
            } if id == window_id => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                mouse_position = (position.x as f32, position.y as f32);
            }
    
            Event::WindowEvent {
                event: WindowEvent::MouseInput { button: winit::event::MouseButton::Left, state, .. },
                ..
            } => {
                left_mouse_button_pressed = state == winit::event::ElementState::Pressed;
            },

            Event::RedrawRequested(_) => {
                simulation.update();

                let particles = simulation.get_particles();
                graphics.render(particles).expect("Failed to render");

                next_frame_time += update_interval;
                graphics.window().request_redraw(); // Important: Requesting the next redraw
            }

            Event::MainEventsCleared => {
                if left_mouse_button_pressed {
                    // This is a repeated action while the left mouse button is held down.
                    let particle = Particle::new(
                        math::vector::Vector2d::new(mouse_position.0, mouse_position.1),
                        math::vector::Vector2d::new(0.0, 0.0),
                        math::vector::Vector2d::new(0.0, 0.0),
                        1.0,
                        10.0, // example radius
                        [1.0, 0.0, 0.0, 1.0], // example color
                    );
                    simulation.add_particle(particle);
                }

                // After handling all events and if the next frame time is in the past, request a redraw
                if Instant::now() >= next_frame_time {
                    graphics.window().request_redraw();
                }
            }

            _ => {}
        }

        *control_flow = ControlFlow::WaitUntil(next_frame_time);
    });

}
fn main() {
    env_logger::init();

    pollster::block_on(run());
}
