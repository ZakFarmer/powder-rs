use crate::math::vector::Vector2d;

use super::particle::Particle;

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub gravity: f32,
    pub time_step: f32,
    pub damping: f32,
    pub bounds: [f32; 4],
}

impl Simulation {
    pub fn new(gravity: f32, time_step: f32, damping: f32, bounds: [f32; 4]) -> Simulation {
        Simulation {
            particles: Vec::new(),
            gravity,
            time_step,
            damping,
            bounds,
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn get_particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    pub fn update(&mut self) {
        for particle in &mut self.particles {
            particle.acceleration.y -= self.gravity;

            particle.velocity.x += particle.acceleration.x * self.time_step;
            particle.velocity.y += particle.acceleration.y * self.time_step;
            particle.position.x += particle.velocity.x * self.time_step;
            particle.position.y += particle.velocity.y * self.time_step;

            particle.acceleration = Vector2d::new(0.0, 0.0);
        }

        for particle in &mut self.particles {
            if particle.position.x - particle.radius < self.bounds[0] {
                particle.velocity.x *= -self.damping;
                particle.position.x = self.bounds[0] + particle.radius;
            } else if particle.position.x + particle.radius > self.bounds[2] {
                particle.velocity.x *= -self.damping;
                particle.position.x = self.bounds[2] - particle.radius;
            }

            if particle.position.y - particle.radius < self.bounds[1] {
                particle.velocity.y *= -self.damping;
                particle.position.y = self.bounds[1] + particle.radius;
            } else if particle.position.y + particle.radius > self.bounds[3] {
                particle.velocity.y *= -self.damping;
                particle.position.y = self.bounds[3] - particle.radius;
            }
        }
    }
}