use crate::misc::Particle;

pub struct World {
    pub particles: Vec<Vec<Option<Particle>>>,
    pub world_width: usize,  // in terms of cell size; relative to cell size
    pub world_height: usize, // ""
    pub cell_width: usize,
    pub cell_height: usize,
}

impl World {
    pub fn new(world_width: usize, world_height: usize, cell_width: usize, cell_height: usize) -> Self {
        let mut particles = vec![vec![None; world_height as usize]; world_width as usize];
        println!("{:#?}", particles);

        World {
            particles,
            world_width,
            world_height,
            cell_width,
            cell_height,
        }
    }
}
