use bevy::prelude::*;

pub mod fireball;

#[derive(Default, Debug, Component)]
pub struct AbilityInstanceMarker;

#[derive(Debug, Clone, Copy, Component)]
pub struct AbilityId(pub Entity);
