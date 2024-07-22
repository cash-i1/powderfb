pub fn particles() -> Vec<Particle> {
    let particles = vec![
        Particle {
            color: 0xfce09f,
            properties: ParticleProperties {
                name: Some("sand".to_string()),
                mass: 1,
                derives: ParticleType::Sand,
                randomness: 50,
            },
        },
        Particle {
            color: 0xf98257,
            properties: ParticleProperties {
                name: Some("brick".to_string()),
                mass: 1,
                derives: ParticleType::Basic,
                randomness: 20,
            },
        },
        Particle {
            color: 0x828282,
            properties: ParticleProperties {
                name: Some("block".to_string()),
                mass: 1,
                derives: ParticleType::Still,
                randomness: 20,
            },
        },
        Particle {
            color: 0x1b55f7,
            properties: ParticleProperties {
                name: Some("water".to_string()),
                mass: 1,
                derives: ParticleType::Water,
                randomness: 10,
            },
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
