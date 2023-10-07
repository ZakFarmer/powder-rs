use super::particle::Particle;

pub struct Simulation {
    pub grid: Vec<Vec<Option<Particle>>>,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            grid: vec![vec![None; height]; width],
        }
    }

    pub fn update(&mut self) {
        for x in 0..self.grid.len() {
            for y in (1..self.grid[0].len()).rev() {
                if let Some(particle) = self.grid[x][y] {
                    self.apply_rules(x, y, particle);
                }
            }
        }
    }

    pub fn spawn_particle(&mut self, x: usize, y: usize, particle: Particle) {
        self.grid[x][y] = Some(particle);
    }

    fn apply_rules(&mut self, x: usize, y: usize, particle: Particle) {
        if x == 0 || x == self.grid.len() - 1 {
            return;
        }

        if y == self.grid[0].len() - 1 {
            return;
        }

        if y > 0 && self.grid[x][y + 1] == None {
            self.grid[x][y] = None;
            self.grid[x][y + 1] = Some(particle);
        }
    }

    pub fn grid(&self) -> &Vec<Vec<Option<Particle>>> {
        &self.grid
    }
}
