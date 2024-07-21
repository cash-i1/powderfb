use crate::misc::Particle;

pub struct World {
    pub particles: Vec<Vec<Option<Particle>>>,
    pub world_width: usize,  // in terms of cell size; relative to cell size
    pub world_height: usize, // ""
    pub cell_width: usize,
    pub cell_height: usize,
    pub game_tick: u32,
    pub selected_particle: Option<Particle>,
}

impl World {
    pub fn new(
        world_width: usize,
        world_height: usize,
        cell_width: usize,
        cell_height: usize,
    ) -> Self {
        let particles = vec![vec![None; world_height as usize]; world_width as usize];

        World {
            particles,
            world_width,
            world_height,
            cell_width,
            cell_height,
            game_tick: 0,
            selected_particle: None,
        }
    }
    pub fn step(&mut self) {
        self.game_tick += 1;

        if self.game_tick % 2 == 0 {
            for i in 0..self.particles.len() {
                for j in (0..self.particles[i].len()).rev() {
                    if let Some(particle) = self.particles[i][j].take() {
                        if j + 1 < self.particles[i].len() && self.particles[i][j + 1].is_none() {
                            self.particles[i][j + 1] = Some(particle);
                        } else {
                            self.particles[i][j] = Some(particle);
                        }
                    } else {
                        self.particles[i][j] = None;
                    }
                }
            }
        }
    }
}
