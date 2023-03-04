use std::time::{Instant, SystemTime};

use cgmath::{Vector2, Zero};

use crate::utils;

use super::physics::gravity::g;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PhysicsType {
    DYNAMIC,
    STATIC,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParticleVariant {
    WOOD,
    STNE,
    URAN,
    PLUT,
    DEUT,
    C4,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
    pub created: u64,
    ///
    pub color: [u8; 4],
    pub physics_type: PhysicsType,
    ///
    pub acceleration: Vector2<f32>,
    pub position_current: Vector2<f32>,
    pub position_old: Vector2<f32>,
    ///
    pub variant: ParticleVariant,
}

impl Particle {
    pub fn new(
        color: [u8; 4],
        physics_type: PhysicsType,
        acceleration: Vector2<f32>,
        position_current: Vector2<f32>,
        position_old: Vector2<f32>,
        variant: ParticleVariant,
    ) -> Self {
        Particle {
            created: utils::time::get_sys_time_in_secs(),
            color,
            physics_type,
            acceleration,
            position_current,
            position_old,
            variant,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let velocity: Vector2<f32> = (self.position_current - self.position_old);

        self.position_old = self.position_current;
        self.position_current =
            (self.position_current + velocity + self.acceleration * delta_time * delta_time);

        self.acceleration = Vector2::new(0.0, g);
    }
}
