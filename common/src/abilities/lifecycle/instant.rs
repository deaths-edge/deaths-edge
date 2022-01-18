use bevy::prelude::*;

use crate::dyn_command::DynCommand;

#[derive(Component)]
pub struct InstantBundle(pub DynCommand);
