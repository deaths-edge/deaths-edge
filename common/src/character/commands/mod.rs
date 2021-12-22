mod action;
mod focal_angle;
mod motion;
mod target;

pub use action::*;
pub use focal_angle::*;
pub use motion::*;
pub use target::*;

use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

pub const CHARACTER_COMMANDS: &str = "character-commands";

/// A character command, addressed by [`Entity`].
pub struct CharacterEntityCommand<Command> {
    pub id: Entity,
    pub command: Command,
}

impl<Command> CharacterEntityCommand<Command> {
    pub fn new(id: Entity, command: Command) -> Self {
        Self { id, command }
    }

    pub fn id(&self) -> Entity {
        self.id
    }

    pub fn command(&self) -> &Command {
        &self.command
    }
}

pub struct CharacterEntityCommandPlugin<T> {
    state: T,
}

impl<T> CharacterEntityCommandPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> Plugin for CharacterEntityCommandPlugin<T>
where
    T: Send + Sync + 'static + Debug + Eq + Hash + Clone + Copy,
{
    fn build(&self, app: &mut AppBuilder) {
        let movement = SystemSet::on_update(self.state)
            .label(CHARACTER_COMMANDS)
            .with_system(character_movement.system())
            .with_system(character_target.system())
            .with_system(character_action.system())
            .with_system(character_focal_rotate.system());
        app.add_event::<CharacterEntityCommand<Motion>>()
            .add_event::<CharacterEntityCommand<Target>>()
            .add_event::<CharacterEntityCommand<Action>>()
            .add_event::<CharacterEntityCommand<FocalAngle>>()
            .add_system_set(movement);
    }
}
