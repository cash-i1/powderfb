use crate::{misc::error, particle::ParticleType, World};

pub struct Simulate {}
impl Simulate {
    pub fn auto(world: &mut World, (i, j): (usize, usize)) {
        if let Some(particle) = &world.particles[i][j] {
            if let Some(properties) = &particle.properties {
                match properties.derives {
                    ParticleType::Still => Self::still(world, (i, j)),
                    _ => error("what"),
                }
            } else if particle.properties.is_none() {
                error("to use `auto`, `properties` has to be `Some`.")
            }
        }
    }
    pub fn still(world: &mut World, (i, j): (usize, usize)) {
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
}
