use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct SpellSource(Entity);

impl From<Entity> for SpellSource {
    fn from(value: Entity) -> Self {
        Self(value)
    }
}

impl PartialEq<Entity> for SpellSource {
    fn eq(&self, other: &Entity) -> bool {
        self.0 == *other
    }
}
