use bevy::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct CharacterTarget(pub Option<Entity>);

impl From<Entity> for CharacterTarget {
    fn from(entity: Entity) -> Self {
        Self(Some(entity))
    }
}

impl CharacterTarget {
    pub fn set_entity(&mut self, character: Entity) -> &mut Self {
        self.0 = Some(character);
        self
    }

    pub fn deselect(&mut self) -> &mut Self {
        self.0 = None;
        self
    }

    pub fn id(&self) -> Option<Entity> {
        self.0
    }
}
