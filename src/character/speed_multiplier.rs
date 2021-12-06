use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct CharacterSpeedMultiplier(f32);

impl From<f32> for CharacterSpeedMultiplier {
    fn from(val: f32) -> Self {
        Self(val)
    }
}

impl Deref for CharacterSpeedMultiplier {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CharacterSpeedMultiplier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CharacterSpeedMultiplier {
    const BASE_SPEED: f32 = 300.;

    pub fn speed(&self) -> f32 {
        Self::BASE_SPEED * self.0
    }
}
