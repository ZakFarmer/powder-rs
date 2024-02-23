use raylib::color::Color;

use crate::GRAVITY;

#[derive(Clone, Copy, Debug)]
pub enum Element {
    Sand,
}

impl Element {
    pub fn color(element: &Element) -> Color {
        match *element {
            Self::Sand => Color::GOLD,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub element: Element,
    velocity: f32,
}

impl Particle {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            velocity: 0.0,
        }
    }

    pub fn color(&self) -> Color {
        Element::color(&self.element)
    }

    pub fn accelerate(&mut self) {
        self.velocity += GRAVITY;
    }

    pub fn move_down(&mut self) {
        self.velocity = (self.velocity - 1.0).max(0.0);
    }
}
