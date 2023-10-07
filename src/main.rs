use std::{time::{Instant, Duration}, sync::{Arc, Mutex}};

use graphics::graphics::{GraphicsContext};
use math::vector::Vector2d;
use midir::{MidiInput, Ignore};
use rodio::{Sink, source::SineWave, Source};
use simulation::{simulation::Simulation, particle::Particle};
use winit::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder, WindowId, WindowAttributes}, event::{Event, WindowEvent}, dpi::LogicalSize};

mod graphics;
mod math;
mod simulation;

pub const SCALE_FACTOR: f32 = 4.;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

async fn run() {
    // Initialise event loop and window
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_title("powder-rs".to_string())
        .build(&event_loop)
        .expect("Failed to create window.");

    let window_id = window.id();

    let mut graphics = GraphicsContext::new(window).await;

    let window_size = graphics.window().inner_size();

    let update_interval = Duration::from_millis(100);

    let mut simulation = Arc::new(Mutex::new(Simulation::new(window_size.width as usize, window_size.height as usize)));

    let mut last_update_time = Instant::now();
    
    let mut left_mouse_button_pressed = false;
    let mut mouse_position = Vector2d::new(0.0, 0.0);

    // Initialise audio
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Initialise MIDI
    let mut midi_in = MidiInput::new("MIDI Keyboard").unwrap();
    midi_in.ignore(Ignore::None);

    let ports = midi_in.ports();
    let in_port = ports.iter().next().unwrap();

    let sim_clone = Arc::clone(&simulation);
    let midi_in_port = midi_in.connect(in_port, "keyboard_port", move |_, message, _| {
        if let &[note_on_status, note, velocity] = message {
            if note_on_status == 144 { // Note-on message on channel 1
                let freq = midi_note_to_freq(note);
                let x = midi_note_to_grid_x(note, WINDOW_WIDTH as usize);

                let mut sim = sim_clone.lock().unwrap();
                sim.spawn_particle(x, 10, Particle::Sand);

                let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
                sink.append(source);
            }
        }
    }, ()).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { 
                event: WindowEvent::CloseRequested, 
                window_id: id,
            } if id == window_id => *control_flow = ControlFlow::Exit,

            Event::RedrawRequested(_) => {
                let sim_clone = Arc::clone(&simulation);
                
                graphics.render(sim_clone).expect("Failed to render");
            },

            Event::MainEventsCleared => {
                let now = Instant::now();
                if now - last_update_time >= update_interval {
                    let mut sim = simulation.lock().unwrap();

                    sim.update();
                    last_update_time = now;
                }

                graphics.window().request_redraw();
            }

            _ => {}
        }
    });

}

fn midi_note_to_freq(note: u8) -> f32 {
    (2f32).powf((note as f32 - 69.0) / 12.0) * 440.0
}

fn midi_note_to_grid_x(midi_note: u8, grid_width: usize) -> usize {
    let clamped_note = midi_note.min(127);
    
    (clamped_note as usize * grid_width) / 128
}

fn main() {
    env_logger::init();

    pollster::block_on(run());
}
