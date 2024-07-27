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
        world.try_move(pos, pos.modify(|p| *p.j_mut() += 1));
    }
    pub fn sand(world: &mut World, pos: Position) {
        let rand_i = pos.i().wrapping_add(if rand::random::<bool>() {
            1
        } else {
            usize::MAX
        });

        world.try_move(pos, pos.modify(|p| *p.j_mut() += 1));
        world.try_move(
            pos,
            pos.modify(|p| {
                *p.i_mut() = rand_i;
                *p.j_mut() += 1
            }),
        );
    }
    pub fn water(world: &mut World, pos: Position) {
        let rand_i = pos.i().wrapping_add(if rand::random::<bool>() {
            1
        } else {
            usize::MAX
        });

        world.try_move(pos, pos.modify(|p| *p.j_mut() += 1));
        world.try_move(
            pos,
            pos.modify(|p| {
                *p.i_mut() = rand_i;
            }),
        );
        world.try_move(
            pos,
            pos.modify(|p| {
                *p.i_mut() = rand_i;
                *p.j_mut() += 1
            }),
        );
    }
    pub fn acid(world: &mut World, pos: Position) {
        let rand_i = pos.i().wrapping_add(if rand::random::<bool>() {
            1
        } else {
            usize::MAX
        });

        world.try_replace(pos, pos.modify(|p| *p.j_mut() += 1));
        if let Some(new_pos) = world.try_replace(
            pos,
            pos.modify(|p| {
                *p.i_mut() = rand_i;
            }),
        ) {
            world.remove(new_pos);
        }
        world.try_replace(
            pos,
            pos.modify(|p| {
                *p.i_mut() = rand_i;
                *p.j_mut() += 1
            }),
        );
    }
}
