use std::time::Duration;

use bevy::prelude::*;

use crate::dyn_command::DynCommand;

#[derive(Debug, Default, Component)]
pub struct CastMarker;

#[derive(Debug, Clone, Component)]
pub struct CastDuration(pub Duration);

#[derive(Component)]
pub struct CastBundle(pub DynCommand);
