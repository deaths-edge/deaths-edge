use bevy::prelude::*;

use crate::spells::SpellTarget;

pub struct EffectTarget(pub Entity);

impl From<SpellTarget> for EffectTarget {
    fn from(value: SpellTarget) -> Self {
        Self(value.0)
    }
}
