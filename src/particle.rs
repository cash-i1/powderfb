use crate::misc::Color;

pub fn particles() -> Vec<Particle> {
    let particles = vec![
        Particle {
            asleep: false,
            color: Color::Custom(0xfce09f),
            properties: ParticleProperties {
                name: Some("sand".to_string()),
                density: 10,
                derives: ParticleType::Sand,
                randomness: 20,
            },
        },
        Particle {
            asleep: false,
            color: Color::Custom(0xf98257),
            properties: ParticleProperties {
                name: Some("brick".to_string()),
                density: 20,
                derives: ParticleType::Basic,
                randomness: 0,
            },
        },
        Particle {
            asleep: false,
            color: Color::Custom(0x828282),
            properties: ParticleProperties {
                name: Some("block".to_string()),
                density: 100,
                derives: ParticleType::Still,
                randomness: 10,
            },
        },
        Particle {
            asleep: false,
            color: Color::Custom(0x1b55f7),
            properties: ParticleProperties {
                name: Some("water".to_string()),
                density: 5,
                derives: ParticleType::Water,
                randomness: 8,
            },
        },
        Particle {
            asleep: false,
            color: Color::Custom(0x44fc1b),
            properties: ParticleProperties {
                name: Some("acid".to_string()),
                density: 5,
                derives: ParticleType::Acid,
                randomness: 10,
            },
        }
    ];
    particles
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum ParticleType {
    Still, // does not move
    Sand,
    Acid,
    Water,
    #[default]
    Basic, // goes down only
}

#[derive(Default, Clone, Debug)]
pub struct ParticleProperties {
    pub name: Option<String>,
    pub density: u32,
    pub derives: ParticleType,
    pub randomness: u32,
}

#[derive(Default, Clone, Debug)]
pub struct Particle {
    pub color: Color,
    pub properties: ParticleProperties,
    pub asleep: bool,
}
