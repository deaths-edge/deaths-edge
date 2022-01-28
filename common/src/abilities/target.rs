use bevy::prelude::*;
/// The target of an [`CharacterEffect`].
#[derive(Debug, Clone, Copy, Component)]
pub struct Target(pub Entity);

/// The source of an [`CharacterEffect`].
#[derive(Debug, Clone, Copy, Component)]
pub struct Source(pub Entity);
