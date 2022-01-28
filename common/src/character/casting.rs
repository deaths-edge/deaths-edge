use std::fmt::Debug;

use bevy::{prelude::*, utils::Instant};

#[derive(Default, Debug, Component)]
pub struct CastState(pub Option<CastRef>);

#[derive(Debug)]
pub struct CastRef {
    pub start: Instant,
    pub cast_id: Entity,
}
