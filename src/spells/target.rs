use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct SpellTarget(Entity);

impl SpellTarget {
    pub fn id(&self) -> Entity {
        self.0
    }
}

impl From<Entity> for SpellTarget {
    fn from(value: Entity) -> Self {
        Self(value)
    }
}

impl From<SpellTarget> for Entity {
    fn from(value: SpellTarget) -> Entity {
        value.0
    }
}

impl PartialEq<Entity> for SpellTarget {
    fn eq(&self, other: &Entity) -> bool {
        self.0 == *other
    }
}
