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
                _ => Self::basic(world, (i, j)),
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
                } else if world.particles[i - 1][j + 1].is_none() {
                    world.particles[i - 1][j + 1] = Some(particle);
                } else if world.particles[i + 1][j + 1].is_none() {
                    world.particles[i + 1][j + 1] = Some(particle);
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
                } else if world.particles[i - 1][j + 1].is_none() {
                    world.particles[i - 1][j + 1] = Some(particle);
                } else if world.particles[i + 1][j + 1].is_none() {
                    world.particles[i + 1][j + 1] = Some(particle);
                } else if world.particles[i - 1][j].is_none() {
                    world.particles[i - 1][j] = Some(particle);
                } else if world.particles[i + 1][j].is_none() {
                    world.particles[i + 1][j] = Some(particle);
                } else {
                    world.particles[i][j] = Some(particle);
                }
            } else {
                world.particles[i][j] = Some(particle);
            }
        }
    }
}
