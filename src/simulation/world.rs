use cgmath::Vector2;
use rand::{rngs::ThreadRng, Rng};

use crate::{utils::{graphics::{Sprite, blit}, geometry::Point}, HEIGHT};

use super::particle::{Particle, ParticleVariant, PhysicsType};

pub struct World {
    y: i32,
    particles: Vec<Particle>,
}

impl World {
    /// Add a particle to the world
    pub fn add_particle(&mut self, x: f32, y: f32, _variant: ParticleVariant) -> bool {
        let mut rng: ThreadRng = rand::thread_rng();

        // Create a new Vector2 for the position
        let position: Vector2<f32> = Vector2::new(x, y);

        // Create a new particle
        self.particles.push(Particle::new(
            crate::COLORS[rng.gen_range(0..7)],
            PhysicsType::DYNAMIC,
            Vector2::new(0.0, 0.0),
            position,
            position,
            ParticleVariant::STNE,
        ));

        true
    }

    /// Clear all particles from the world
    pub fn clear_particles(&mut self) -> bool {
        self.particles.clear();

        true
    }

    pub fn new() -> Self {
        Self {
            y: 0,
            particles: vec![],
        }
    }

    pub fn draw(&self, frame: &mut [u8], sprite: &Sprite) {
        // Draw the particles
        for particle in &self.particles {
            if particle.position_current.y >= HEIGHT as f32 {
                continue;
            }
            
            let x = particle.position_current.x as usize;
            let y = particle.position_current.y as usize;
            //let r = particle.radius as u32;

            let color = particle.color;
            let pos = Point::new(x, y);

            blit(frame, &pos, sprite, color);
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.y == 200 {
            self.y = 0;
        } else {
            self.y = self.y + 1;
        }

        for particle in &mut self.particles {
            if particle.position_current.y >= HEIGHT as f32 {
                self.particles = self.particles.retain(|p| p.created != particle.created);
                continue;
            }

            particle.update(delta_time);
        }
    }
}
