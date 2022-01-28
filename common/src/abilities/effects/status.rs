use bevy::prelude::*;

use crate::{
    abilities::{AbilityId, Target},
    dyn_command::DynEntityMutate,
};

use super::CharacterEffect;

pub struct ApplyStatus(pub DynEntityMutate);

impl CharacterEffect for ApplyStatus {
    type Domain<'a> = Entity;

    type Param<'w, 's> = ();

    type Fetch = ();

    fn apply(
        &self,
        _time: &Time,
        _ability_id: &AbilityId,
        item: Entity,
        _param: &(),
        commands: &mut Commands,
    ) {
        let mut entity_commands = commands.spawn();
        self.0.apply(&mut entity_commands);
        entity_commands.insert(Target(item));
    }
}
