mod ability;
mod focal_angle;
mod motion;
mod select;

pub use ability::*;
pub use focal_angle::*;
pub use motion::*;
pub use select::*;

use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

pub const CHARACTER_COMMANDS: &str = "character-commands";

/// A character action, addressed by [`Entity`].
pub struct CharacterEntityAction<Action> {
    pub id: Entity,
    pub action: Action,
}

impl<Action> CharacterEntityAction<Action> {
    pub fn new(id: Entity, action: Action) -> Self {
        Self { id, action }
    }

    pub fn id(&self) -> Entity {
        self.id
    }

    pub fn action(&self) -> &Action {
        &self.action
    }
}

pub struct CharacterEntityActionPlugin<T> {
    state: T,
}

impl<T> CharacterEntityActionPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> Plugin for CharacterEntityActionPlugin<T>
where
    T: Send + Sync + 'static + Debug + Eq + Hash + Clone + Copy,
{
    fn build(&self, app: &mut App) {
        let movement = SystemSet::on_update(self.state)
            .label(CHARACTER_COMMANDS)
            .with_system(character_movement)
            .with_system(character_target)
            .with_system(character_ability)
            .with_system(character_focal_rotate);
        app.add_event::<CharacterEntityAction<Motion>>()
            .add_event::<CharacterEntityAction<SelectTarget>>()
            .add_event::<CharacterEntityAction<Ability>>()
            .add_event::<CharacterEntityAction<FocalAngle>>()
            .add_system_set(movement);
    }
}
