use bevy::prelude::*;

use super::{CastOrAbilityFilter, Obstruction, UseObstructions};
use crate::{
    abilities::{Source, Target},
    character::{CharacterMarker, Team},
};

/// Ability requires a target.
#[derive(Debug, Clone, Component)]
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
    // TODO: Add/Removal detection
    character_query: Query<(Option<&Target>, &Team), With<CharacterMarker>>,
    mut ability_query: Query<(&Source, &RequiresTarget, &mut UseObstructions), CastOrAbilityFilter>,
    target_query: Query<&Team, With<CharacterMarker>>,
) {
    for (source, requires_target, mut obstructions) in ability_query.iter_mut() {
        if let Ok((target, self_team)) = character_query.get(source.0) {
            if let Some(&Target(target_id)) = target {
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
