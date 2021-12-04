use bevy::prelude::*;

#[derive(Debug)]
pub struct NameplateParent(Entity);

impl NameplateParent {
    pub fn id(&self) -> Entity {
        self.0
    }
}

impl Into<Entity> for NameplateParent {
    fn into(self) -> Entity {
        self.0
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
