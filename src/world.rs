use crate::misc::Rectangle;
use crate::particle::Particle;
use crate::simulate::Simulate;
use crate::Graphics;

pub struct World {
    pub particles: Vec<Vec<Option<Particle>>>,
    pub world_width: usize,  // in terms of cell size; relative to cell size
    pub world_height: usize, // ""
    pub cell_width: usize,
    pub cell_height: usize,
    pub game_tick: u32,
    pub selected_particle: Option<Particle>,
    pub paused: bool,
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
            paused: false,
        }
    }
    pub fn step(&mut self) {
        if self.paused {return;}
        self.game_tick += 1;

        if self.game_tick % 2 == 0 {
            for i in 0..self.particles.len() {
                for j in (0..self.particles[i].len()).rev() {
                    Simulate::auto(self, (i, j));
                }
            }
        }
    }
    pub fn draw(&self, gfx: &mut Graphics) {
        for (x, i) in self.particles.iter().enumerate() {
            for (y, particle) in i.iter().enumerate() {
                if let Some(p) = particle {
                    let rect = Rectangle {
                        x: self.cell_width * x,
                        y: self.cell_height * y,
                        width: self.cell_width,
                        height: self.cell_height,
                        color: p.color,
                    };
                    gfx.rectangle(rect);
                }
            }
        }
    }
}
