use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct CastMarker;

#[derive(Component)]
pub struct CastBundle(pub fn() -> Box<dyn ApplicableBundle>);
