use bevy::{
    prelude::*,
    utils::{HashMap, Instant},
};

use crate::abilities::MagicType;

#[derive(Default, Debug, Component)]
pub struct Interrupts(pub HashMap<MagicType, Instant>);

impl Interrupts {
    pub fn is_locked(&self, spell_type: &MagicType) -> bool {
        self.0.contains_key(&spell_type)
    }
}
