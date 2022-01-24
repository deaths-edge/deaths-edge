use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct AbilityName(pub &'static str);

#[derive(Debug, Component)]
pub struct AbilityDescription(pub &'static str);
