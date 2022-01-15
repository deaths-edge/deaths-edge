use bevy::prelude::*;

use crate::character::{CharacterMarker, OptionalTarget, Team};

use super::{
    AbilityId, AbilityInstanceMarker, AbilityMarker, CharacterId, Obstruction, Preparing,
    UseObstructions,
};

/// Ability requires a target.
#[derive(Debug, Component)]
pub enum RequiresTarget {
    Enemy,
    Ally,
    Both,
}

impl Default for RequiresTarget {
    fn default() -> Self {
        Self::Enemy
    }
}

/// Checks the required target is correct.
pub fn check_required_target(
    mut ability_query: Query<
        (&CharacterId, &RequiresTarget, &mut UseObstructions),
        With<AbilityMarker>,
    >,
    character_query: Query<
        (&OptionalTarget, &Team),
        (With<CharacterMarker>, Changed<OptionalTarget>),
    >,
    target_query: Query<&Team, With<CharacterMarker>>,
) {
    for (source, requires_target, mut obstructions) in ability_query.iter_mut() {
        if let Ok((target, self_team)) = character_query.get(source.0) {
            if let Some(target_id) = target.0 {
                obstructions.0.remove(&Obstruction::NoTarget);

                let target_team = target_query.get(target_id).expect("failed to find target");

                let correct_target = match requires_target {
                    RequiresTarget::Enemy => target_team != self_team,
                    RequiresTarget::Ally => target_team == self_team,
                    RequiresTarget::Both => true,
                };

                if correct_target {
                    obstructions.0.remove(&Obstruction::IncorrectTarget);
                } else {
                    obstructions.0.insert(Obstruction::IncorrectTarget);
                }
            } else {
                obstructions.0.insert(Obstruction::NoTarget);
            }
        }
    }
}

/// The target of the ability.
#[derive(Debug, Clone, Copy, Component)]
pub struct Target(pub Entity);

/// Adds current target to instance ability.
pub fn adjoin_target(
    instance_query: Query<(Entity, &AbilityId), (With<Preparing>, With<AbilityInstanceMarker>)>,
    ability_query: Query<&CharacterId, With<AbilityMarker>>,
    character_query: Query<&OptionalTarget, With<CharacterMarker>>,

    mut commands: Commands,
) {
    for (instance_id, ability_id) in instance_query.iter() {
        let source = ability_query
            .get(ability_id.0)
            .expect("failed to find ability");

        let opt_target = character_query
            .get(source.0)
            .expect("failed to find character");

        commands.entity(instance_id).insert(Target(
            opt_target.0.expect("this cannot be empty due to checks"),
        ));
    }
}
