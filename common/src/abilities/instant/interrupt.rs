use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{
        AbilityId, AbilityInstanceMarker, AbilityMarker, Casting, Complete, Failed, MagicType,
        Target,
    },
    character::{CastState, CharacterMarker, Interrupts},
};

pub struct InstantInterrupt(pub Duration);

pub fn apply_interrupt(
    time: Res<Time>,

    interrupt_instance_query: Query<
        (&AbilityId, &Target),
        (With<Complete>, With<AbilityInstanceMarker>),
    >,
    interrupt_ability_query: Query<&InstantInterrupt, With<AbilityMarker>>,

    cast_instance_query: Query<(Entity, &AbilityId), (With<Casting>, With<AbilityInstanceMarker>)>,
    cast_ability_query: Query<&MagicType, With<AbilityMarker>>,

    mut character_query: Query<(&mut CastState, &mut Interrupts), With<CharacterMarker>>,

    mut commands: Commands,
) {
    let now = time.last_update().expect("failed to find last update");

    for (ability_id, target) in interrupt_instance_query.iter() {
        let interrupt_ability_query = interrupt_ability_query
            .get(ability_id.0)
            .expect("failed to find ability");

        let (mut cast_state, mut interrupts) = character_query
            .get_mut(target.0)
            .expect("failed to find character");

        if let Some(cast) = cast_state.0.take() {
            let (cast_id, ability_id) = cast_instance_query
                .get(cast.instance_id)
                .expect("failed to find cast");
            let cast_spell_type = cast_ability_query
                .get(ability_id.0)
                .expect("failed to find ability");

            interrupts
                .0
                .insert(*cast_spell_type, now + interrupt_ability_query.0);

            commands.entity(cast_id).remove::<Casting>().insert(Failed);
        }
    }
}
