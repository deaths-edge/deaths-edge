use std::time::Duration;

pub struct SpellMarker;

pub struct SpellIndex(usize);

impl From<usize> for SpellIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub enum Spell {
    Fireball,
}

impl Spell {
    pub fn duration(&self) -> Duration {
        use Spell::*;

        match self {
            Fireball => Duration::from_secs(2),
        }
    }
}
