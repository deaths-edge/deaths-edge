mod ability_cooldown;
mod cant_while_casting;
mod global_cooldown;
mod maximum_range;
mod power;
mod requires_fov;
mod requires_los;
mod requires_stationary;
mod requires_target;

use std::{fmt::Debug, hash::Hash};

pub use ability_cooldown::*;
pub use cant_while_casting::*;
pub use global_cooldown::*;
pub use maximum_range::*;
pub use power::*;
pub use requires_fov::*;
pub use requires_los::*;
pub use requires_stationary::*;
pub use requires_target::*;

use bevy::{prelude::*, utils::HashSet};

use super::{lifecycle::CastMarker, AbilityMarker};

#[derive(Debug, Clone, Default, Component)]
pub struct UseObstructions(pub HashSet<Obstruction>);

/// An obstruction preventing a specific ability from being used.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Obstruction {
    InsufficientPower,
    OutOfRange,
    OutOfFOV,
    OutOfLoS,
    NoTarget,
    IncorrectTarget,
    Cooldown,
    GlobalCooldown,
    NonStationary,
    Locked,
    Casting,
}

type CastOrAbilityFilter = Or<(With<AbilityMarker>, With<CastMarker>)>;

pub struct ObstructionPlugin<T, L> {
    pub state: T,
    pub label: L,
}

impl<T, L> Plugin for ObstructionPlugin<T, L>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Eq + Hash,

    L: Send + Sync + 'static,
    L: SystemLabel + Clone,
{
    fn build(&self, app: &mut App) {
        let ability_checks = SystemSet::on_update(self.state)
            .label(self.label.clone())
            // Geometric obstructions
            .with_system(check_required_target)
            .with_system(check_required_fov)
            .with_system(check_maximum_range)
            .with_system(check_required_stationary)
            // Resource obstructions
            .with_system(check_power_cost)
            // Cooldown obstructions
            .with_system(check_global_cooldown)
            .with_system(check_cooldown)
            // Casting obstructions
            .with_system(check_while_casting);
        app.add_system_set(ability_checks);
    }
}
