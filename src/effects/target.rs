use bevy::prelude::*;

use crate::spells::SpellTarget;

pub struct EffectTarget(Entity);

impl EffectTarget {
    pub fn id(&self) -> Entity {
        self.0
    }
}

impl From<Entity> for EffectTarget {
    fn from(value: Entity) -> Self {
        Self(value)
    }
}

impl Into<Entity> for EffectTarget {
    fn into(self) -> Entity {
        self.0
    }
}

impl From<SpellTarget> for EffectTarget {
    fn from(value: SpellTarget) -> Self {
        let value: Entity = value.into();
        value.into()
    }
}

impl PartialEq<Entity> for EffectTarget {
    fn eq(&self, other: &Entity) -> bool {
        self.0 == *other
    }
}
