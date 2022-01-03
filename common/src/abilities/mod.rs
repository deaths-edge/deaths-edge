mod cast_duration;
mod cooldown;
mod global_cooldown;
mod health_cost;
mod instances;
mod maximum_range;
mod power_cost;
mod requires_fov;
mod requires_stationary;
mod requires_target;
mod spell_type;

pub use cast_duration::*;
pub use cooldown::*;
pub use global_cooldown::*;
pub use health_cost::*;
pub use instances::*;
pub use maximum_range::*;
pub use power_cost::*;
pub use requires_fov::*;
pub use requires_stationary::*;
pub use requires_target::*;
pub use spell_type::*;

use std::{fmt::Debug, hash::Hash, time::Duration};

use bevy::{prelude::*, utils::HashSet};

////
// These components will be in the platonic "ability"
// They are attached to each character via `AbilitySource`.

pub struct AbilityMarker;

/// The character which the ability originates from.
#[derive(Debug)]
pub struct AbilitySource(pub Entity);

////
// After the input (or network) event triggers an ability, a new "instance" of the ability is
// created using the components below.

#[derive(PartialEq, Eq, Hash)]
pub enum Obstruction {
    InsufficientPower,
    OutOfRange,
    OutOfFOV,
    NoTarget,
    IncorrectTarget,
    Cooldown,
    GlobalCooldown,
    NonStationary,
    Locked,
}

pub struct UseObstructions(HashSet<Obstruction>);

pub struct AbilityPlugin<T> {
    state: T,
}

impl<T> Plugin for AbilityPlugin<T>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Hash + Eq,
{
    fn build(&self, app: &mut AppBuilder) {
        let ability_checks = SystemSet::on_update(self.state)
            // Geometric obstructions
            .with_system(check_required_target.system())
            .with_system(check_required_fov.system())
            .with_system(check_maximum_range.system())
            .with_system(check_required_stationary.system())
            // Resource obstructions
            .with_system(check_power_cost.system())
            // Cooldown obstructions
            .with_system(check_global_cooldown.system())
            .with_system(check_cooldown.system())
            // Check silence/lock
            // .with_system()
            ;

        let apply_cost = SystemSet::on_update(self.state)
            .with_system(apply_health_cost.system())
            .with_system(apply_power_cost.system());

        app.add_system_set(ability_checks)
            .add_system_set(apply_cost);
    }
}
