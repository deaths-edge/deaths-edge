use bevy::prelude::*;
/// The target of an [`Effect`].
#[derive(Debug, Clone, Copy, Component)]
pub struct Target(pub Entity);

/// The source of an [`Effect`].
#[derive(Debug, Clone, Component)]
pub struct Source(pub Entity);
