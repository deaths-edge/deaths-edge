use bevy::prelude::*;

use crate::{
    abilities::{AbilityId, Target},
    dyn_command::DynEntityMutate,
};

use super::CharacterEffect;

#[derive(Clone)]
pub struct ApplyStatus(pub DynEntityMutate);

impl CharacterEffect for ApplyStatus {
    type Domain<'a> = Entity;

    type Param<'w, 's> = ();

    type Fetch = ();

    fn apply(
        &self,
        _time: &Time,
        ability_id: &AbilityId,
        item: Entity,
        _param: &(),
        commands: &mut Commands,
    ) {
        info!("applying status");
        let mut entity_commands = commands.spawn();
        self.0.apply(&mut entity_commands);
        entity_commands.insert(Target(item)).insert(*ability_id);
    }
}
