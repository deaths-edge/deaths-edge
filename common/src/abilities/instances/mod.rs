mod fireblast;
mod kick;
mod scorch;

use bevy::prelude::Component;
pub use fireblast::*;
pub use kick::*;
pub use scorch::*;

use crate::dyn_command::DynEntityMutate;

#[derive(Debug, Component)]
pub struct OnPress(pub DynEntityMutate);
