mod ability_cooldown;
mod global_cooldown;
mod maximum_range;
mod power;
mod requires_fov;
mod requires_los;
mod requires_stationary;
mod requires_target;

pub use ability_cooldown::*;
pub use global_cooldown::*;
pub use maximum_range::*;
pub use power::*;
pub use requires_fov::*;
pub use requires_los::*;
pub use requires_stationary::*;
pub use requires_target::*;

use bevy::{prelude::*, utils::HashSet};

#[derive(Debug, Default, Component)]
pub struct UseObstructions(pub HashSet<Obstruction>);

/// An obstruction preventing a specific ability from being used.
#[derive(Debug, PartialEq, Eq, Hash)]
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
}
