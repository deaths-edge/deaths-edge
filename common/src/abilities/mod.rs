mod cooldown;
mod global_cooldown;
mod health_cost;
mod instances;
mod power_cost;
mod requires_target;

pub use cooldown::*;
pub use global_cooldown::*;
pub use instances::*;
pub use power_cost::*;
pub use requires_target::*;

use std::time::Duration;

use bevy::{
    prelude::*,
    utils::{HashSet, Instant},
};

////
// These components will be in the platonic "ability"
// They are attached to each character via `AbilitySource`.

pub struct AbilityMarker;

/// The character which the ability originates from.
pub struct AbilitySource(pub Entity);

/// Requires that target is in Field of View.
pub struct RequiresFov;

/// Ability has a maximum range.
pub struct MaximumRange(pub f32);

/// Ability requires casting duration.
pub struct CastDuration(Duration);

////
// After the input (or network) event triggers an ability, a new "instance" of the ability is
// created using the components below.

pub struct AbilityInstanceMarker;

pub struct AbilityInstance(pub Entity);

#[derive(PartialEq, Eq, Hash)]
pub enum Obstruction {
    InsuffientPower,
    OutOfRange,
    OutOfFOV,
    NoTarget,
    IncorrectTarget,
    Cooldown,
    GlobalCooldown,
}

pub struct UseObstructions(HashSet<Obstruction>);
