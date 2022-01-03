use bevy::prelude::*;

use crate::character::{CharacterMarker, Target, Team};

use super::{AbilityMarker, AbilitySource, Obstruction, UseObstructions};

/// Ability requires a target.
pub enum RequiresTarget {
    Enemy,
    Ally,
    Both,
}

/// Checks the required target is correct.
pub fn check_required_target(
    mut ability_query: Query<
        (&AbilitySource, &RequiresTarget, &mut UseObstructions),
        With<AbilityMarker>,
    >,
    character_query: Query<(&Target, &Team), (With<CharacterMarker>, Changed<Target>)>,
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
