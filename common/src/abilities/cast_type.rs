use std::time::Duration;

use bevy::prelude::*;

use crate::character::{Cast, CastState, CharacterMarker};

use super::{AbilityInstance, AbilityMarker, AbilitySource};

/// Ability requires casting duration.
pub enum CastType {
    Instant,
    Cast(Duration),
    Channel(Duration),
}
