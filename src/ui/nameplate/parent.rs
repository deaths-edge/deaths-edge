use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct NameplateParent(Entity);

impl NameplateParent {
    pub fn id(&self) -> Entity {
        self.0
    }
}

impl From<NameplateParent> for Entity {
    fn from(value: NameplateParent) -> Entity {
        value.0
    }
}

impl From<Entity> for NameplateParent {
    fn from(value: Entity) -> Self {
        Self(value)
    }
}

impl PartialEq<Entity> for NameplateParent {
    fn eq(&self, other: &Entity) -> bool {
        self.0 == *other
    }
}
