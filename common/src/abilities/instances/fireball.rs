use crate::abilities::*;

#[derive(Bundle)]
pub struct Fireball {
    marker: AbilityMarker,

    global_cooldown: GlobalCooldown,

    power_cost: PowerCost,

    requires_target: RequiresTarget,
    requires_fov: RequiresFov,
    requires_los: RequiresLoS,
    max_range: MaximumRange,

    cast_type: CastType,
}

// pub struct FireballImpact {
//     marker: AbilityMarker,
//     target_damage: InstantDamage,
// }

impl Fireball {
    pub fn new() -> Self {
        Self {
            marker: AbilityMarker,

            global_cooldown: GlobalCooldown,

            power_cost: PowerCost(20.),

            requires_target: RequiresTarget::Enemy,
            requires_fov: RequiresFov,
            requires_los: RequiresLoS,
            max_range: MaximumRange(500.),

            cast_type: CastType::Cast(Duration::from_secs(1)),
        }
    }
}
