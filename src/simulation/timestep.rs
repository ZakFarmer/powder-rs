use std::time::Instant;

pub const MS_PER_UPDATE: f32 = 4.0;

#[derive(Debug)]
pub struct TimeStep {
    last_time: Instant,
    delta_time: f32,
    frame_count: u32,
    frame_time: f32,
}

impl TimeStep {
    pub fn new() -> TimeStep {
        Self {
            last_time: Instant::now(),
            delta_time: 0.0,
            frame_count: 0,
            frame_time: 0.0,
        }
    }

    pub fn delta(&mut self) -> f32 {
        let current_time: Instant = Instant::now();

        let delta: f32 = current_time.duration_since(self.last_time).as_micros() as f32 * 0.001;

        self.last_time = current_time;
        self.delta_time = delta;

        delta
    }

    pub fn fps(&mut self) -> Option<u32> {
        self.frame_count += 1;
        self.frame_time += self.delta_time;

        let tmp;

        if self.frame_time >= 1000.0 {
            tmp = self.frame_count;

            self.frame_count = 0;
            self.frame_time = 0.0;

            return Some(tmp);
        }

        None
    }
}
