use std::sync::{Arc, Mutex};

use cgmath::Vector2;
use lazy_static::lazy_static;
use quadtree_rs::{Quadtree, area::{AreaBuilder, Area}, point::Point, entry::Entry, iter::Query};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    config::CONFIG,
    utils::{
        graphics::{blit, Sprite},
    },
    HEIGHT, simulation::physics::gravity::g, WIDTH,
};

use super::{particle::{Particle, ParticleVariant, PhysicsType}};

pub struct World {
    culling_area: Area<u64>,
    safe_area: Area<u64>,
    ///
    particle_tree: Quadtree<u64, Particle>,
}

impl World {
    /// Add a particle to the world
    pub fn add_particle(&mut self, x: f32, y: f32, _variant: ParticleVariant) -> bool {
        let mut rng: ThreadRng = rand::thread_rng();

        // Create a new Vector2 for the position
        let position: Vector2<f32> = Vector2::new(x, y);

        let particle = Particle::new(
            crate::COLORS[rng.gen_range(0..7)],
            PhysicsType::DYNAMIC,
            Vector2::new(0.0, g),
            position,
            position,
            ParticleVariant::STNE,
        );

        let particle_region: Area<u64> = AreaBuilder::default()
            .anchor(Point {x: x as u64, y: y as u64})
            .dimensions((1, 1))
            .build().unwrap();

        let val = self.particle_tree.insert(particle_region, particle);

        true
    }

    /// Clear all particles from the world
    pub fn clear_particles(&mut self) -> () {
        self.particle_tree.reset()
    }

    pub fn new() -> Self {
        let culling_area: Area<u64> = AreaBuilder::default()
            .anchor(Point {x: 0, y: HEIGHT as u64 - 1})
            .dimensions((WIDTH as u64, 99999))
            .build().unwrap();

        let safe_area: Area<u64> = AreaBuilder::default()
            .anchor(Point {x: 0, y: 0})
            .dimensions((WIDTH as u64, HEIGHT as u64))
            .build().unwrap();

        let depth: usize = 12;
        let quadtree: Quadtree<u64, Particle> = Quadtree::new(depth);

        Self {
            culling_area,
            safe_area,
            particle_tree: quadtree,
        }
    }

    pub fn draw(&self, frame: &mut [u8], sprite: &Sprite) {
        // Draw the particles
        for node in self.particle_tree.iter() {
            let particle: Particle = node.value_ref().clone();

            if particle.position_current.y >= HEIGHT as f32 {
                continue;
            }

            let x: usize = particle.position_current.x as usize;
            let y: usize = particle.position_current.y as usize;

            let color: [u8; 4] = particle.color;
            let pos = crate::utils::geometry::Point::new(x, y);

            blit(frame, &pos, sprite, color);
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.particle_tree.retain(|p| {
            if p.position_current.y >= HEIGHT as f32 {
                return true;
            }

            false
        });

        self.particle_tree.modify(self.safe_area, |p| {        
            let velocity: Vector2<f32> = p.position_current - p.position_old;

            p.position_old = p.position_current;

            p.position_current = p.position_current
                + velocity 
                + p.acceleration 
                * delta_time 
                * delta_time;

            p.acceleration = Vector2::new(0.0, g);
        });
    }
}
