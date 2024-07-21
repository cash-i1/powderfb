pub fn particles() -> Vec<Particle> {
    let particles = vec![
        Particle {
            color: 11122,
            properties: ParticleProperties {
                name: None,
                mass: 1,
                derives: ParticleType::Sand,
                randomness: 20,
            },
        },
        Particle {
            color: 99992,
            properties: ParticleProperties::default(),
        },
        Particle {
            color: 556600,
            properties: ParticleProperties::default(),
        },
        Particle {
            color: 22222,
            properties: ParticleProperties::default(),
        },
        Particle {
            color: 0xfce09f,
            properties: ParticleProperties::default(),
        }
    ];
    particles
}

#[derive(Default, Clone, Debug)]
pub enum ParticleType {
    Still, // does not move
    Sand,
    Water,
    #[default]
    Basic, // goes down only
}

#[derive(Default, Clone, Debug)]
pub struct ParticleProperties {
    pub name: Option<String>,
    pub mass: u32,
    pub derives: ParticleType,
    pub randomness: u32,
}

#[derive(Default, Clone, Debug)]
pub struct Particle {
    pub color: u32,
    pub properties: ParticleProperties,
}
