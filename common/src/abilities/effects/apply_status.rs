use bevy::prelude::*;

use crate::dyn_command::DynEntityMutate;

use super::CharacterEffect;

#[derive(Clone)]
pub struct ApplyStatus(pub DynEntityMutate);

impl CharacterEffect for ApplyStatus {
    type Domain<'a> = ();

    type Param<'w, 's> = ();

    type Fetch = ();

    fn apply(
        &self,
        parent_id: Entity,
        _item: (),
        _param: &(),

        _time: &Time,

        commands: &mut Commands,
    ) {
        info!("applying status");
        let mut entity_commands = commands.spawn();
        self.0.apply(parent_id, &mut entity_commands);
    }
}
