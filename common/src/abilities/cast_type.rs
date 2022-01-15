use std::time::Duration;

use bevy::prelude::*;

/// Ability requires casting duration.
#[derive(Debug, Component)]
pub enum CastType {
    Instant,
    Cast(Duration),
    Channel(Duration),
}

impl Default for CastType {
    fn default() -> Self {
        Self::Instant
    }
}
