use std::{fmt::Debug, hash::Hash};

use bevy::{core::Time, prelude::*, utils::Instant};
use heron::rapier_plugin::PhysicsWorld;

use crate::abilities::AbilitySource;

use super::CharacterMarker;

#[derive(Default, Debug)]
pub struct CastState {
    pub ability_id: Option<Cast>,
}

#[derive(Debug)]
pub struct Cast {
    pub start: Instant,
    pub ability_id: Entity,
}
