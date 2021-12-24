use bevy::prelude::*;

use super::CharacterMarker;

#[derive(Debug)]
pub struct CharacterPower {
    pub current: f32,
    pub total: f32,
}

pub struct CharacterPowerRegen(pub f32);

/// Regenerates power over time.
pub fn regenerate_power(
    time: Res<Time>,

    mut query: Query<(&CharacterPowerRegen, &mut CharacterPower), With<CharacterMarker>>,
) {
    let delta_time = time.delta_seconds();

    for (regen, mut power) in query.iter_mut() {
        power.current += delta_time * regen.0;
        power.current = power.current.clamp(0., power.total)
    }
}
