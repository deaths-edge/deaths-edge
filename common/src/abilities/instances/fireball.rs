use std::time::Duration;

use crate::abilities::*;

#[derive(Bundle)]
pub struct Fireball {
    marker: AbilityMarker,

    global_cooldown: GlobalCooldown,

    power_cost: PowerCost,

    requires_target: RequiresTarget,
    requires_stationary: RequiresStationary,
    requires_fov: RequiresFov,
    requires_los: RequiresLoS,
    max_range: MaximumRange,

    instant_damage: InstantDamage,

    cast_type: CastType,
}

impl Fireball {
    pub fn new() -> Self {
        Self {
            marker: AbilityMarker,

            global_cooldown: GlobalCooldown,

            power_cost: PowerCost(20.),

            requires_target: RequiresTarget::Enemy,
            requires_stationary: RequiresStationary,
            requires_fov: RequiresFov,
            requires_los: RequiresLoS,
            max_range: MaximumRange(500.),

            instant_damage: InstantDamage(25.),
            cast_type: CastType::Cast(Duration::from_secs(1)),
        }
    }
}
