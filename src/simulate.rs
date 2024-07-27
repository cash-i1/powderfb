use crate::misc::Direction;
use rand::Rng;

use crate::{
    misc::{error, pos, Position},
    particle::ParticleType,
    World,
};

pub struct Simulate {}
impl Simulate {
    pub fn auto(world: &mut World, pos: Position) {
        if let Some(particle) = &world.particles[pos.i()][pos.j()] {
            match particle.properties.derives {
                ParticleType::Basic => Self::basic(world, pos),
                ParticleType::Sand => Self::sand(world, pos),
                ParticleType::Water => Self::water(world, pos),
                ParticleType::Acid => Self::acid(world, pos),
                ParticleType::Still => {}
            }
        }
    }
    pub fn basic(world: &mut World, pos: Position) {
        if let Some(particle) = world.take(pos) {
            if let Some(new_pos) = world.can_move(pos, Direction::down()) {
                world.set(new_pos, particle);
            } else {
                world.set(pos, particle);
            }
        }
    }
    pub fn sand(world: &mut World, pos: Position) {
        if let Some(particle) = world.particles[pos.i()][pos.j()].take() {
            let rand_i = pos.i().wrapping_add(if rand::random::<bool>() {
                1
            } else {
                usize::MAX
            });

            if pos.j().checked_add(1).is_some()
                && pos.i().checked_sub(1).is_some()
                && pos.i().checked_add(1).is_some()
                && pos.j() + 1 < world.particles[pos.i()].len()
                && pos.i() + 1 < world.particles.len()
            {
                if pos.j() == world.particles[pos.i()].len() {
                    world.particles[pos.i()][pos.j()] = Some(particle);
                } else if world.particles[pos.i()][pos.j() + 1].is_none() {
                    world.particles[pos.i()][pos.j() + 1] = Some(particle);
                } else if world.particles[rand_i][pos.j() + 1].is_none() {
                    world.particles[rand_i][pos.j() + 1] = Some(particle);
                } else {
                    world.particles[pos.i()][pos.j()] = Some(particle);
                }
            } else {
                world.particles[pos.i()][pos.j()] = Some(particle);
            }
        }
    }
    pub fn water(world: &mut World, pos: Position) {
        if let Some(particle) = world.particles[pos.i()][pos.j()].take() {
            let rand_i = pos.i().wrapping_add(if rand::random::<bool>() {
                1
            } else {
                usize::MAX
            });

            if pos.j().checked_add(1).is_some()
                && pos.i().checked_sub(1).is_some()
                && pos.i().checked_add(1).is_some()
                && pos.j() + 1 < world.particles[pos.i()].len()
                && pos.i() + 1 < world.particles.len()
            {
                if pos.j() == world.particles[pos.i()].len() {
                    world.particles[pos.i()][pos.j()] = Some(particle);
                } else if world.particles[pos.i()][pos.j() + 1].is_none() {
                    world.particles[pos.i()][pos.j() + 1] = Some(particle);
                } else if world.particles[rand_i][pos.j() + 1].is_none() {
                    world.particles[rand_i][pos.j() + 1] = Some(particle);
                } else if world.particles[rand_i][pos.j()].is_none() {
                    world.particles[rand_i][pos.j()] = Some(particle);
                } else {
                    world.particles[pos.i()][pos.j()] = Some(particle);
                }
            } else {
                world.particles[pos.i()][pos.j()] = Some(particle);
            }
        }
    }
    pub fn acid(world: &mut World, pos: Position) {
        if let Some(particle) = world.particles[pos.i()][pos.j()].take() {
            let rand_i = pos.i().wrapping_add(if rand::random::<bool>() {
                1
            } else {
                usize::MAX
            });

            if pos.j().checked_add(1).is_some()
                && pos.i().checked_sub(1).is_some()
                && pos.i().checked_add(1).is_some()
                && pos.j() + 1 < world.particles[pos.i()].len()
                && pos.i() + 1 < world.particles.len()
            {
                if pos.j() == world.particles[pos.i()].len() {
                    world.particles[pos.i()][pos.j()] = Some(particle);
                } else if world.particles[pos.i()][pos.j() + 1].is_some()
                    && world.particles[pos.i()][pos.j() + 1].is_some()
                    && world.particles[pos.i()][pos.j() + 1]
                        .clone()
                        .unwrap()
                        .properties
                        .derives
                        != ParticleType::Acid
                {
                    world.particles[pos.i()][pos.j() + 1] = Some(particle);
                } else if world.particles[rand_i][pos.j() + 1].is_some()
                    && world.particles[pos.i()][pos.j() + 1].is_some()
                    && world.particles[pos.i()][pos.j() + 1]
                        .clone()
                        .unwrap()
                        .properties
                        .derives
                        != ParticleType::Acid
                {
                    world.particles[rand_i][pos.j() + 1] = Some(particle);
                } else if world.particles[rand_i][pos.j()].is_some()
                    && world.particles[pos.i()][pos.j() + 1].is_some()
                    && world.particles[pos.i()][pos.j() + 1]
                        .clone()
                        .unwrap()
                        .properties
                        .derives
                        != ParticleType::Acid
                {
                    world.particles[rand_i][pos.j()] = Some(particle);
                } else if world.particles[pos.i()][pos.j() + 1].is_none() {
                    world.particles[pos.i()][pos.j() + 1] = Some(particle);
                } else if world.particles[rand_i][pos.j() + 1].is_none() {
                    world.particles[rand_i][pos.j() + 1] = Some(particle);
                } else if world.particles[rand_i][pos.j()].is_none() {
                    world.particles[rand_i][pos.j()] = Some(particle);
                } else {
                    world.particles[pos.i()][pos.j()] = Some(particle);
                }
            } else {
                world.particles[pos.i()][pos.j()] = Some(particle);
            }
        }
    }
}
