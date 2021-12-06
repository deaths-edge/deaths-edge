use bevy::prelude::*;

#[derive(Default, Debug, Clone, Copy, Component)]
pub struct CharacterTarget {
    // TODO: Remove pub
    pub target: Option<Entity>,
}

impl From<Entity> for CharacterTarget {
    fn from(entity: Entity) -> Self {
        Self {
            target: Some(entity),
        }
    }
}

impl CharacterTarget {
    pub fn set_entity(&mut self, character: Entity) -> &mut Self {
        self.target = Some(character);
        self
    }

    pub fn deselect(&mut self) -> &mut Self {
        self.target = None;
        self
    }

    pub fn id(&self) -> Option<Entity> {
        self.target
    }
}
