pub fn particles() -> Vec<Particle> {
    let particles = vec![
        Particle {
            color: 11122,
            properties: Some(ParticleProperties {
                name: String::from(""),
                mass: 1,
                derives: ParticleType::Still,
            }),
        },
        Particle {
            color: 99992,
            properties: Some(ParticleProperties::default()),
        },
        Particle {
            color: 556600,
            properties: Some(ParticleProperties::default()),
        },
        Particle {
            color: 22222,
            properties: Some(ParticleProperties::default()),
        },
    ];
    particles
}

#[derive(Default, Clone, Debug)]
pub enum ParticleType {
    #[default]
    Still, // does not move
    Sand,
    Water,
    Basic, // goes down only
}

#[derive(Default, Clone, Debug)]
pub struct ParticleProperties {
    pub name: String,
    pub mass: u32,
    pub derives: ParticleType,
}

#[derive(Default, Clone, Debug)]
pub struct Particle {
    pub color: u32,
    pub properties: Option<ParticleProperties>,
}
