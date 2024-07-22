use rand::Rng;

use crate::{misc::error, particle::ParticleType, World};

pub struct Simulate {}
impl Simulate {
    pub fn auto(world: &mut World, (i, j): (usize, usize)) {
        if let Some(particle) = &world.particles[i][j] {
            match particle.properties.derives {
                ParticleType::Basic => Self::basic(world, (i, j)),
                ParticleType::Sand => Self::sand(world, (i, j)),
                ParticleType::Still => {}
                ParticleType::Water => Self::water(world, (i, j)),
                ParticleType::Acid => Self::acid(world, (i, j)),
            }
        }
    }
    pub fn basic(world: &mut World, (i, j): (usize, usize)) {
        if let Some(particle) = world.particles[i][j].take() {
            if j + 1 < world.particles[i].len() && world.particles[i][j + 1].is_none() {
                world.particles[i][j + 1] = Some(particle);
            } else {
                world.particles[i][j] = Some(particle);
            }
        } else {
            world.particles[i][j] = None;
        }
    }
    pub fn sand(world: &mut World, (i, j): (usize, usize)) {
        if let Some(particle) = world.particles[i][j].take() {
            let rand_i = i.wrapping_add(if rand::random::<bool>() {
                1
            } else {
                usize::MAX
            });

            if j.checked_add(1).is_some()
                && i.checked_sub(1).is_some()
                && i.checked_add(1).is_some()
                && j + 1 < world.particles[i].len()
                && i + 1 < world.particles.len()
            {
                if j == world.particles[i].len() {
                    world.particles[i][j] = Some(particle);
                } else if world.particles[i][j + 1].is_none() {
                    world.particles[i][j + 1] = Some(particle);
                } else if world.particles[rand_i][j + 1].is_none() {
                    world.particles[rand_i][j + 1] = Some(particle);
                } else {
                    world.particles[i][j] = Some(particle);
                }
            } else {
                world.particles[i][j] = Some(particle);
            }
        }
    }
    pub fn water(world: &mut World, (i, j): (usize, usize)) {
        if let Some(particle) = world.particles[i][j].take() {
            let rand_i = i.wrapping_add(if rand::random::<bool>() {
                1
            } else {
                usize::MAX
            });

            if j.checked_add(1).is_some()
                && i.checked_sub(1).is_some()
                && i.checked_add(1).is_some()
                && j + 1 < world.particles[i].len()
                && i + 1 < world.particles.len()
            {
                if j == world.particles[i].len() {
                    world.particles[i][j] = Some(particle);
                } else if world.particles[i][j + 1].is_none() {
                    world.particles[i][j + 1] = Some(particle);
                } else if world.particles[rand_i][j + 1].is_none() {
                    world.particles[rand_i][j + 1] = Some(particle);
                } else if world.particles[rand_i][j].is_none() {
                    world.particles[rand_i][j] = Some(particle);
                } else {
                    world.particles[i][j] = Some(particle);
                }
            } else {
                world.particles[i][j] = Some(particle);
            }
        }
    }
    pub fn acid(world: &mut World, (i, j): (usize, usize)) {
        if let Some(particle) = world.particles[i][j].take() {
            let rand_i = i.wrapping_add(if rand::random::<bool>() {
                1
            } else {
                usize::MAX
            });

            if j.checked_add(1).is_some()
                && i.checked_sub(1).is_some()
                && i.checked_add(1).is_some()
                && j + 1 < world.particles[i].len()
                && i + 1 < world.particles.len()
            {
                if j == world.particles[i].len() {
                    world.particles[i][j] = Some(particle);
                } else if world.particles[i][j + 1].is_some()
                    && world.particles[i][j + 1].is_some()
                    && world.particles[i][j + 1]
                        .clone()
                        .unwrap()
                        .properties
                        .derives
                        != ParticleType::Acid
                {
                    world.particles[i][j + 1] = Some(particle);
                } else if world.particles[rand_i][j + 1].is_some()
                    && world.particles[i][j + 1].is_some()
                    && world.particles[i][j + 1]
                        .clone()
                        .unwrap()
                        .properties
                        .derives
                        != ParticleType::Acid
                {
                    world.particles[rand_i][j + 1] = Some(particle);
                } else if world.particles[rand_i][j].is_some()
                    && world.particles[i][j + 1].is_some()
                    && world.particles[i][j + 1]
                        .clone()
                        .unwrap()
                        .properties
                        .derives
                        != ParticleType::Acid
                {
                    world.particles[rand_i][j] = Some(particle);
                } else if world.particles[i][j + 1].is_none() {
                    world.particles[i][j + 1] = Some(particle);
                } else if world.particles[rand_i][j + 1].is_none() {
                    world.particles[rand_i][j + 1] = Some(particle);
                } else if world.particles[rand_i][j].is_none() {
                    world.particles[rand_i][j] = Some(particle);
                } else {
                    world.particles[i][j] = Some(particle);
                }
            } else {
                world.particles[i][j] = Some(particle);
            }
        }
    }
}
