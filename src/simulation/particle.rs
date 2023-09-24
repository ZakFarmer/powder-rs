use crate::math::vector::Vector2d;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Particle {
    pub position: Vector2d,
    pub velocity: Vector2d,
    pub acceleration: Vector2d,
    pub mass: f32,
    pub radius: f32,
    pub color: [f32; 4],
}

impl Particle {
    pub fn new(position: Vector2d, velocity: Vector2d, acceleration: Vector2d, mass: f32, radius: f32, color: [f32; 4]) -> Particle {
        Particle {
            position,
            velocity,
            acceleration,
            mass,
            radius,
            color,
        }
    }
}

impl Default for Particle {
    fn default() -> Particle {
        Particle {
            position: Vector2d::new(0.0, 0.0),
            velocity: Vector2d::new(0.0, 0.0),
            acceleration: Vector2d::new(0.0, 0.0),
            mass: 1.0,
            radius: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}