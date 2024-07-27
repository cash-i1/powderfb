use crate::misc::{pos, Direction, Position, Rectangle};
use crate::particle::{Particle, ParticleType};
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
        if self.paused {
            return;
        }
        self.game_tick += 1;

        if self.game_tick % 2 == 0 {
            for i in 0..self.particles.len() {
                for j in (0..self.particles[i].len()).rev() {
                    Simulate::auto(self, pos(i, j));
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
                    };
                    gfx.rectangle(rect, p.color);
                }
            }
        }
    }

    pub fn can_move(&self, position: Position, direction: Direction) -> Option<Position> {
        let new_i = position.i() as i32 + direction.i();
        let new_j = position.j() as i32 + direction.j();

        if new_i >= 0
            && new_i < self.particles.len() as i32
            && new_j >= 0
            && new_j < self.particles[position.i()].len() as i32
        {
            if self.particles[new_i as usize][new_j as usize].is_none() {
                return Some(pos(new_i as usize, new_j as usize));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    pub fn try_move(&mut self, position1: Position, position2: Position) {
        if let Some(particle) = self.take(position1) {
            if self.is_available(position2) {
                self.set(position2, particle);
                self.wake_at(position2);
            } else {
                self.set(position1, particle);
            }
        }
    }
    /// particle at `position1` is moved to `position2`, even if `position2` has a particle there already
    /// returns the new position if successful or returns None if not successful
    pub fn try_replace(&mut self, position1: Position, position2: Position) -> Option<Position> {
        if let Some(particle) = self.take(position1) {
            if self.is_valid(position2) {
                self.set(position2, particle);
                return Some(position2.clone());
            } else {
                self.set(position1, particle);
                return None;
            }
        } else {
            return None;
        }
    }
    pub fn is_valid(&self, position: Position) -> bool {
        position.i() < self.particles.len() && position.j() < self.particles[position.i()].len()
    }

    pub fn wake_at(&self, position: Position) {

    }
    pub fn take(&mut self, position: Position) -> Option<Particle> {
        self.particles[position.i()][position.j()].take()
    }
    pub fn is_empty(&self, position: Position) -> bool {
        if let Some(_) = self.particles.get(position.i()) {
            if let Some(_) = self.particles[position.i()].get(position.j()) {
                if self.particles[position.i()][position.j()].is_none() {
                    return true;
                } else {
                    return false;
                }
            }
        }
        false
    }
    pub fn particle_at(&mut self, position: Position) -> Option<&mut Particle> {
        self.particles[position.i()][position.j()].as_mut()
    }
    pub fn set(&mut self, position: Position, particle: Particle) {
        self.particles[position.i()][position.j()] = Some(particle);
    }
    pub fn is_available(&self, position: Position) -> bool {
        if position.i() < self.particles.len() && position.j() < self.particles[position.i()].len()
        {
            if let Some(_particle) = &self.particles[position.i()][position.j()] {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }
    pub fn remove(&mut self, position: Position) {
        if self.is_valid(position) {
            self.particles[position.i()][position.j()] = None;
        }
    }
}
