use bevy::prelude::*;

use crate::dyn_command::DynCommand;

#[derive(Debug, Clone, Component)]
pub struct InstantBundle(pub DynCommand);
