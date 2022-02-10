use bevy::prelude::*;

use crate::abilities::AbilityMarker;

/// Complaining does nothing.
#[derive(Debug, Bundle)]
pub struct Complain {
    marker: AbilityMarker,
}

impl Complain {
    pub fn new() -> Self {
        Self {
            marker: AbilityMarker,
        }
    }
}
