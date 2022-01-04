use std::fmt::Debug;

use bevy::{prelude::*, utils::Instant};

#[derive(Default, Debug)]
pub struct CastState(pub Option<Cast>);

#[derive(Debug)]
pub struct Cast {
    pub start: Instant,
    pub cast_id: Entity,
}
