use crate::abilities::*;

#[derive(Bundle)]
pub struct Fireball {
    marker: AbilityMarker,

    global_cooldown: GlobalCooldown,

    power_cost: PowerCost,

    requires_target: RequiresTarget,
    cast_duration: CastType,
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
            cast_duration: CastType::Cast(Duration::from_secs(1)),
        }
    }
}
