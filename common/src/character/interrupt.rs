use bevy::utils::{HashMap, Instant};

use crate::abilities::SpellType;

#[derive(Default)]
pub struct Interrupts(pub HashMap<SpellType, Instant>);

impl Interrupts {
    pub fn is_locked(&self, spell_type: &SpellType) -> bool {
        self.0.contains_key(&spell_type)
    }
}
