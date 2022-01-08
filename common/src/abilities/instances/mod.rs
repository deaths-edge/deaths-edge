use bevy::prelude::*;

pub mod fireball;

#[derive(Default, Debug)]
pub struct AbilityInstanceMarker;

#[derive(Debug, Clone, Copy)]
pub struct AbilityId(pub Entity);
