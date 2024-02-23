use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

use super::particle::Particle;

struct ParticleMoveInfo {
    index: usize,
    particle: Particle,
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Option<Particle>>,
}

impl Grid {
    pub fn clear(&mut self) {
        self.cells.fill(None)
    }

    pub fn draw(&self, draw: &mut RaylibDrawHandle) {
        for (index, cell) in self.cells.iter().enumerate() {
            if cell.is_some() {
                let color = cell.unwrap().color();

                let x = index % self.width;
                let y = index / self.width;

                println!("Cell XY: {x} {y}");

                draw.draw_rectangle(x as i32, y as i32, 1, 1, color)
            }
        }
    }

    pub fn update(&mut self) {
        let mut new_cells = vec![None; self.cells.len()];
        let mut particles_to_move: Vec<ParticleMoveInfo> = Vec::new();

        for (index, cell) in self.cells.iter().enumerate() {
            if let Some(particle) = cell {
                let below_index = self.below_index(index);
                if self.is_empty(below_index) {
                    particles_to_move.push(ParticleMoveInfo {
                        index: below_index,
                        particle: particle.clone(),
                    });
                } else {
                    particles_to_move.push(ParticleMoveInfo {
                        index: index,
                        particle: particle.clone(),
                    });
                }
            }
        }

        for ParticleMoveInfo {
            index,
            mut particle,
        } in particles_to_move
        {
            particle.move_down();
            self.cells[index] = Some(particle);
        }

        self.cells = new_cells;
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        if x >= self.width || x <= 0 || y >= self.height || y <= 0 {
            return 0;
        }

        y * self.width + x
    }

    fn below_index(&self, index: usize) -> usize {
        self.index(index % self.width, index / self.width + 1)
    }

    pub fn is_empty(&self, index: usize) -> bool {
        self.cells[index].is_none()
    }

    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![None; width * height];

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, particle: Particle) {
        let index = self.index(x, y);

        if index <= self.cells.len() {
            self.cells[index] = Some(particle)
        }
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.cells.swap(a, b)
    }
}
