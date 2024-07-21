use crate::World;

pub struct Simulate {}
impl Simulate {
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
