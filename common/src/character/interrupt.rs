use bevy::{prelude::*, utils::Instant};

#[derive(Debug, Component)]
pub struct Interrupt<School> {
    school: School,
    until: Instant,
}
