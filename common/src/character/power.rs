use bevy::prelude::*;

use super::CharacterMarker;

#[derive(Debug, Component)]
pub struct Power {
    pub current: f32,
    pub total: f32,
}

impl Power {
    pub fn subtract(&mut self, cost: f32) {
        self.current -= cost;
        self.current = self.current.min(0.);
    }
}

#[derive(Debug, Component)]
pub struct PowerRegenerate(pub f32);

/// Regenerates power over time.
pub fn regenerate_power(
    time: Res<Time>,

    mut query: Query<(&PowerRegenerate, &mut Power), With<CharacterMarker>>,
) {
    let delta_time = time.delta_seconds();

    for (regen, mut power) in query.iter_mut() {
        power.current += delta_time * regen.0;
        power.current = power.current.clamp(0., power.total)
    }
}
