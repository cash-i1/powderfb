pub const PARTICLES: [Particle; 4] = [
    Particle {
        color: 11122,
        properties: None,
    },
    Particle {
        color: 99992,
        properties: None,
    },
    Particle {
        color: 556600,
        properties: None,
    },
    Particle {
        color: 22222,
        properties: None,
    },
];

#[derive(Default, Clone, Debug)]
pub enum ParticleRoots {
    #[default]
    Sand,
    Water,
    Static, // does not move
    Basic, // goes down only
}

#[derive(Default, Clone, Debug)]
pub struct ParticleProperties {
    name: String,
    mass: u32,
    derives: ParticleRoots
}

#[derive(Default, Clone, Debug)]
pub struct Particle {
    pub color: u32,
    pub properties: Option<ParticleProperties>,
}
