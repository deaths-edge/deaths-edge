use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Health {
    pub current: f32,
    pub total: f32,
}

impl Health {
    pub fn apply_damage(&mut self, damage: f32) -> &mut Self {
        self.current = (self.current - damage).max(0.);
        self
    }

    pub fn apply_heal(&mut self, heal: f32) -> &mut Self {
        self.current = self.total.min(self.current + heal);
        self
    }
}

#[derive(Debug, Component)]
pub struct HealthRegenerate(pub f32);

/// Regenerates health over time.
pub fn regenerate_health(time: Res<Time>, mut query: Query<(&HealthRegenerate, &mut Health)>) {
    let delta_time = time.delta_seconds();

    for (regen, mut power) in query.iter_mut() {
        power.current += delta_time * regen.0;
        power.current = power.current.clamp(0., power.total)
    }
}
