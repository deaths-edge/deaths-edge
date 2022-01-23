use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerState {
    Waiting,
    Spawned,
    Dead,
}

#[derive(Default, Debug, Component)]
pub struct PlayerMarker;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(PlayerState::Waiting);
    }
}
