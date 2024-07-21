use crate::{misc::error, particle::ParticleType, World};

pub struct Simulate {}
impl Simulate {
    pub fn auto(world: &mut World, (i, j): (usize, usize)) {
        if let Some(particle) = &world.particles[i][j] {
            if let Some(properties) = &particle.properties {
                match properties.derives {
                    ParticleType::Basic => Self::basic(world, (i, j)),
                    ParticleType::Sand => Self::sand(world, (i, j)),
                    _ => Self::basic(world, (i, j)),
                }
            } else if particle.properties.is_none() {
                error("to use `auto`, `properties` has to be `Some`.")
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
            if j + 1 < world.particles[i].len() && world.particles[i][j + 1].is_none() {
                world.particles[i][j + 1] = Some(particle);
            } else if i.checked_sub(1).unwrap_or(0) < world.particles.len()
                && j + 1 < world.particles[i].len()
                && world.particles[i.checked_sub(1).unwrap_or(0)][j+1].is_none()
            {
                world.particles[i.checked_sub(1).unwrap_or(0)][j + 1] = Some(particle);
            } else if i + 1 < world.particles.len()
                && j + 1 < world.particles[i].len()
                && world.particles[i + 1][j+1].is_none()
            {
                world.particles[i + 1][j + 1] = Some(particle);
            } else {
                world.particles[i][j] = Some(particle);
            }
        } else {
            world.particles[i][j] = None;
        }
    }
}
