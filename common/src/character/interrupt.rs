use std::marker::PhantomData;

use bevy::{prelude::*, utils::Instant};

#[derive(Debug, Component)]
pub struct Interrupted<School> {
    _school: PhantomData<School>,
    until: Instant,
}

impl<School> Interrupted<School> {
    pub fn new(until: Instant) -> Self {
        Self {
            _school: PhantomData,
            until,
        }
    }
}
