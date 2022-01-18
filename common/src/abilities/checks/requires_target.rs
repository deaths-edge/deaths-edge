use bevy::prelude::*;

use super::{Obstruction, UseObstructions};
use crate::{
    abilities::{AbilityId, AbilityMarker},
    character::{Abilities, CharacterMarker, OptionalTarget, Team},
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
    character_query: Query<
        (&Abilities, &OptionalTarget, &Team),
        (With<CharacterMarker>, Changed<OptionalTarget>),
    >,
    mut ability_query: Query<(&RequiresTarget, &mut UseObstructions), With<AbilityMarker>>,
    target_query: Query<&Team, With<CharacterMarker>>,
) {
    for (abilities, target, self_team) in character_query.iter() {
        for AbilityId(ability_id) in *abilities {
            if let Ok((requires_target, mut obstructions)) = ability_query.get_mut(ability_id) {
                if let Some(target_id) = target.0 {
                    obstructions.0.remove(&Obstruction::NoTarget);

                    let target_team = target_query
                        .get(target_id.0)
                        .expect("failed to find target");

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
}
